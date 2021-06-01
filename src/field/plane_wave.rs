use std::f64::consts;
use rand::prelude::*;
use rand_distr::StandardNormal;

use crate::field::{Field, Polarization};
use crate::constants::*;
use crate::geometry::FourVector;
use crate::nonlinear_compton;
use crate::pair_creation;

/// Represents the envelope of a plane-wave laser pulse, i.e.
/// the field after cycle averaging
pub struct PlaneWave {
    a0: f64,
    n_cycles: f64,
    wavevector: FourVector,
    pol: Polarization,
    chirp_b: f64,
    bandwidth: f64,
}

impl PlaneWave {
    #[allow(unused)]
    pub fn new(a0: f64, wavelength: f64, n_cycles: f64, pol: Polarization, chirp_b: f64) -> Self {
        let wavevector = (2.0 * consts::PI / wavelength) * FourVector::new(1.0, 0.0, 0.0, 1.0);
        PlaneWave {
            a0,
            n_cycles,
            wavevector,
            pol,
            chirp_b,
            bandwidth: 0.0,
        }
    }

    pub fn with_finite_bandwidth(self) -> Self {
        let mut cpy = self;
        // n_fwhm = 2 n acos[1/2^(1/4)] / pi
        let n_fwhm = 0.36405666377387671305 * cpy.n_cycles;
        cpy.bandwidth = (0.5 * consts::LN_2).sqrt() / (consts::PI * n_fwhm);
        cpy
    }
    
    #[allow(unused)]
    pub fn k(&self) -> FourVector {
        self.wavevector
    }

    pub fn a_sqd(&self, r: FourVector) -> f64 {
        let norm = match self.pol {
            Polarization::Linear => 0.5,
            Polarization::Circular => 1.0,
        };
        let phase = self.wavevector * r;
        if phase.abs() < consts::PI * self.n_cycles {
            // a = a0 {sin(phi), cos(phi)} cos[phi/(2n)]^2
            norm * self.a0.powi(2) * (phase / (2.0 * self.n_cycles)).cos().powi(4)
        } else {
            0.0
        }
    }

    pub fn grad_a_sqd(&self, r: FourVector) -> FourVector {
        let norm = match self.pol {
            Polarization::Linear => 0.5,
            Polarization::Circular => 1.0,
        };
        let phase = self.wavevector * r;
        let grad = if phase.abs() < consts::PI * self.n_cycles {
            norm * self.wavevector[0] * self.a0.powi(2) * (phase/self.n_cycles).sin() * (phase/(2.0 * self.n_cycles)).cos().powi(2) / self.n_cycles
        } else {
            0.0
        };
        FourVector::new(
            0.0,
            0.0,
            0.0,
            grad,
        )
    }
}

impl Field for PlaneWave {
    fn total_energy(&self) -> f64 {
        0.0
    }

    fn max_timestep(&self) -> Option<f64> {
        Some( 1.0 / (SPEED_OF_LIGHT * self.wavevector[0]) )
    }

    fn contains(&self, r: FourVector) -> bool {
        let phase = self.wavevector * r;
        phase < consts::PI * self.n_cycles
    }

    fn push(&self, r: FourVector, u: FourVector, rqm: f64, dt: f64) -> (FourVector, FourVector) {
        // equations of motion are:
        //   du/dt = -c grad<a^2>(r) / (2 gamma) = f(r, u)
        //   dr/dt = u c / gamma = g(u)
        // where gamma^2 = 1 + <a^2> + |u|^2, i.e. u * u = 1 + <a^2>

        let scale = (rqm / (ELECTRON_CHARGE / ELECTRON_MASS)).powi(2);

        let f = |r: FourVector, u: FourVector| -> FourVector {
            -scale * SPEED_OF_LIGHT * self.grad_a_sqd(r) / (2.0 * u[0])
        };

        let g = |u: FourVector| -> FourVector {
            SPEED_OF_LIGHT * u / u[0]
        };

        // Heun's method, construct intermediate values
        let r_inter: FourVector = r + dt * g(u);
        let u_inter: FourVector = u + dt * f(r, u);
        let u_inter: FourVector = u_inter.with_sqr(1.0 + self.a_sqd(r_inter));

        // And corrected final values
        let r_new = r + 0.5 * dt * (g(u) + g(u_inter));
        let u_new = u + 0.5 * dt * (f(r, u) + f(r_inter, u_inter));
        let u_new = u_new.with_sqr(1.0 + self.a_sqd(r_new));

        (r_new, u_new)
    }

    fn radiate<R: Rng>(&self, r: FourVector, u: FourVector, dt: f64, rng: &mut R) -> Option<(FourVector, FourVector)> {
        let phase = self.wavevector * r;
        let chirp = if cfg!(feature = "compensating-chirp") {
            1.0 + self.chirp_b * self.a_sqd(r)
        } else {
            //1.0 + 2.0 * self.chirp_b * (phase + consts::PI * self.n_cycles) // alt convention
            1.0 + 2.0 * self.chirp_b * phase
        };
        if chirp < 0.0 && u * u > 1.0 { // frequency must be positive if local a > 0
            assert!(chirp > 0.0, "The specified chirp coefficient of {:.3e} causes the local frequency (eta/eta_0 = {:.3e}) at phase = {:.3} to fall below zero!", self.chirp_b, chirp, self.wavevector * r);
        }
        let width = 1.0 + self.bandwidth * rng.sample::<f64,_>(StandardNormal);
        assert!(width > 0.0, "The fractional bandwidth of the pulse, {:.3e}, is large enough that the sampled frequency has fallen below zero!", self.bandwidth);
        let kappa = SPEED_OF_LIGHT * COMPTON_TIME * self.wavevector * chirp * width;
        let prob = nonlinear_compton::probability(kappa, u, dt).unwrap_or(0.0);
        if rng.gen::<f64>() < prob {
            let (n, k) = nonlinear_compton::generate(kappa, u, rng, None);
            Some((k, u + (n as f64) * kappa - k))
        } else {
            None
        }
    }

    fn pair_create<R: Rng>(&self, r: FourVector, ell: FourVector, dt: f64, rng: &mut R, rate_increase: f64) -> (f64, Option<(FourVector, FourVector)>) {
        let a = self.a_sqd(r).sqrt();
        let phase: f64 = self.wavevector * r;
        let chirp = if cfg!(feature = "compensating-chirp") {
            1.0 + self.chirp_b * a * a
        } else {
            1.0 + 2.0 * self.chirp_b * phase
        };
        if chirp < 0.0 && a > 0.0 { // frequency must be positive if local a > 0
            assert!(chirp > 0.0, "The specified chirp coefficient of {:.3e} causes the local frequency (eta/eta_0 = {:.3e}) at phase = {:.3} to fall below zero!", self.chirp_b, chirp, self.wavevector * r);
        }
        let kappa = SPEED_OF_LIGHT * COMPTON_TIME * self.wavevector * chirp;
        let prob = pair_creation::probability(ell, kappa, a, dt).unwrap_or(0.0);
        if rng.gen::<f64>() < prob * rate_increase {
            let (n, q_p) = pair_creation::generate(ell, kappa, a, rng);
            (prob, Some((ell + (n as f64) * kappa - q_p, q_p)))
        } else {
            (prob, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_wave_cp() {
        let n_cycles = 8.0;
        let wavelength = 0.8e-6;
        let t_start = -0.5 * n_cycles * wavelength / (SPEED_OF_LIGHT);
        let dt = 0.25 * 0.8e-6 / (SPEED_OF_LIGHT);
        let laser = PlaneWave::new(100.0, wavelength, n_cycles, Polarization::Circular, 0.0);

        let mut u = FourVector::new(0.0, 0.0, 0.0, -1000.0).unitize();
        let mut r = FourVector::new(0.0, 0.0, 0.0, 0.0) + u * SPEED_OF_LIGHT * t_start / u[0];
        
        for _k in 0..2 {
            for _i in 0..16 {
                let new = laser.push(r, u, ELECTRON_CHARGE / ELECTRON_MASS, dt);
                r = new.0;
                u = new.1;
            }
            println!("phase = 2 pi {:.3}, u_perp = ({:.3e}, {:.3e}), uz = {:.6e}, u^2 = 1 + {:.6e}", laser.k() * r / (2.0 * consts::PI), u[1], u[2], u[3], u * u - 1.0);
        }

        assert!(u[1] < 1.0e-3);
        assert!(u[2] < 1.0e-3);
        assert!((u * u - 1.0).abs() < 1.0e-3);
    }
}