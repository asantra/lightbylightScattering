//! Representation of the electromagnetic field in the simulation domain

use rand::prelude::*;
use crate::geometry::FourVector;

mod focused_laser;
mod fast_focused_laser;
mod plane_wave;
mod fast_plane_wave;

pub use self::focused_laser::*;
pub use self::fast_focused_laser::*;
pub use self::plane_wave::*;
pub use self::fast_plane_wave::*;

/// The polarization of an electromagnetic wave
#[allow(unused)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "hdf5-output", derive(hdf5::H5Type))]
#[repr(u8)]
pub enum Polarization {
    Linear = 0,
    Circular = 1,
}

/// Represents the electromagnetic field in a spatiotemporal domain.
pub trait Field {
    /// Returns the total electromagnetic energy in joules
    fn total_energy(&self) -> f64;

    /// Returns the largest usuable value of the timestep `dt`
    /// used in the particle push, or `None` if there is no
    /// particular restriction
    fn max_timestep(&self) -> Option<f64>;

    /// Is the specified four-position within the field?
    fn contains(&self, r: FourVector) -> bool;

    /// Advances the position `r` and normalized momentum `u`
    /// of a particle with charge to mass ratio `rqm`
    /// by a timestep `dt`, returning a tuple of the new
    /// position and momentum, as well as the change in
    /// lab time (which may differ from `dt`)
    fn push(&self, r: FourVector, u: FourVector, rqm: f64, dt: f64) -> (FourVector, FourVector, f64);

    /// Checks to see whether an electron in the field, located at
    /// position `r` with momentum `u` emits a photon, and if so,
    /// returns the momentum of that photon, the new momentum of the electron,
    /// and the effective a0 of the interaction.
    fn radiate<R: Rng>(&self, r: FourVector, u: FourVector, dt: f64, rng: &mut R) -> Option<(FourVector, FourVector, f64)>;

    /// Checks to see if an electron-positron pair is produced by
    /// a photon (position `r`, normalized momentum `ell`), returning the
    /// probability that it occurs in the specified interval `dt` and,
    /// if so, the fraction of the photon that decays, the
    /// the momentum of the electron and positron, and the effective
    /// a0 of the interaction.
    ///
    /// A non-unity `rate_increase` makes pair creation more probable
    /// by the given factor, increasing the statistics for what would
    /// otherwise be a rare event. The probability returned is *not*
    /// affected by this increase.
    fn pair_create<R: Rng>(&self, r: FourVector, ell: FourVector, dt: f64, rng: &mut R, rate_increase: f64) -> (f64, f64, Option<(FourVector, FourVector, f64)>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn cp_deflection() {
        let fast_laser = FastFocusedLaser::new(100.0, 0.8e-6, 4.0e-6, 30.0e-15, Polarization::Circular);
        let laser = FocusedLaser::new(100.0, 0.8e-6, 4.0e-6, 30.0e-15, Polarization::Circular);

        let t_start = -20.0 * 0.8e-6 / (SPEED_OF_LIGHT);
        let x0 = 2.0e-6;

        let u = FourVector::new(0.0, 0.0, 0.0, -1000.0).unitize();
        let r = FourVector::new(0.0, x0, 0.0, 0.0) + u * SPEED_OF_LIGHT * t_start / u[0];

        // ponderomotive solver
        let dt = 0.5 * 0.8e-6 / (SPEED_OF_LIGHT);
        let mut pond = (r, u, dt);
        for _i in 0..(20*2*2) {
            pond = laser.push(pond.0, pond.1, ELECTRON_CHARGE / ELECTRON_MASS, dt);
        }
        let pond = pond.1;

        // Lorentz force solve
        // ponderomotive solver
        let dt = 0.01 * 0.8e-6 / (SPEED_OF_LIGHT);
        let mut lorentz = (r, u, dt);
        for _i in 0..(20*2*100) {
            lorentz = fast_laser.push(lorentz.0, lorentz.1, ELECTRON_CHARGE / ELECTRON_MASS, dt);
        }
        let lorentz = lorentz.1;

        let theory = 2.0 * 3.63189;
        let pond_angle = 1.0e3 * pond[1].atan2(-pond[3]);
        let lorentz_angle = 1.0e3 * lorentz[1].atan2(-lorentz[3]);
        let error = ((pond_angle - lorentz_angle) / lorentz_angle).abs();

        println!("angle [PF] = {:.3e}, angle [LF] = {:.3e}, error = {:.3}%, predicted = {:.3e}", pond_angle, lorentz_angle, 100.0 * error, theory);
        assert!(error < 1.0e-2);
    }

    #[test]
    fn lp_deflection() {
        let fast_laser = FastFocusedLaser::new(100.0, 0.8e-6, 4.0e-6, 30.0e-15, Polarization::Linear);
        let laser = FocusedLaser::new(100.0, 0.8e-6, 4.0e-6, 30.0e-15, Polarization::Linear);

        let t_start = -20.0 * 0.8e-6 / (SPEED_OF_LIGHT);
        let y0 = 2.0e-6;

        let u = FourVector::new(0.0, 0.0, 0.0, -1000.0).unitize();
        let r = FourVector::new(0.0, 0.0, y0, 0.0) + u * SPEED_OF_LIGHT * t_start / u[0];

        // ponderomotive solver
        let dt = 0.5 * 0.8e-6 / (SPEED_OF_LIGHT);
        let mut pond = (r, u, dt);
        for _i in 0..(20*2*2) {
            pond = laser.push(pond.0, pond.1, ELECTRON_CHARGE / ELECTRON_MASS, dt);
        }
        let pond = pond.1;

        // Lorentz force solve
        // ponderomotive solver
        let dt = 0.01 * 0.8e-6 / (SPEED_OF_LIGHT);
        let mut lorentz = (r, u, dt);
        for _i in 0..(20*2*100) {
            lorentz = fast_laser.push(lorentz.0, lorentz.1, ELECTRON_CHARGE / ELECTRON_MASS, dt);
        }
        let lorentz = lorentz.1;

        let theory = 3.63189;
        let pond_angle = 1.0e3 * pond[2].atan2(-pond[3]);
        let lorentz_angle = 1.0e3 * lorentz[2].atan2(-lorentz[3]);
        let error = ((pond_angle - lorentz_angle) / lorentz_angle).abs();

        println!("angle [PF] = {:.3e}, angle [LF] = {:.3e}, error = {:.3}%, predicted = {:.3e}", pond_angle, lorentz_angle, 100.0 * error, theory);
        assert!(error < 1.0e-2);
    }

}