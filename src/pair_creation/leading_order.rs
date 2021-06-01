//! Rates and spectra for pair creation in the limit a << 1,
//! i.e. ignoring mass-shift effects

use super::*;

fn partial_spectrum(n: i32, a: f64, eta: f64, s: f64) -> f64 {
    let j = n as f64;
    let sn = 2.0 * j * eta;
    let zeta = {
        let tmp = 1.0 / (sn * s * (1.0 - s));
        (4.0 * j * j * tmp * (1.0 - tmp)).sqrt()
    };
    let prefactor = 4.0_f64.powi(-n) * (a * zeta).powi(2 * n) / (n-1).factorial().powi(2);
    if !prefactor.is_finite() {
        0.0
    } else {
        prefactor * (j.powi(-2) + zeta.powi(-2) * (1.0 / (s * (1.0 - s)) - 2.0))
    }
}

fn partial_rate(n: i32, a: f64, eta: f64) -> f64 {
    let ell = n as f64;
    let sn = 2.0 * ell * eta;
    if sn <= 4.0 {
        return 0.0;
    }
    let s_min = 0.5 - (0.25 - 1.0 / sn).sqrt();
    let s_max = 0.5;
    let rate: f64 = GAUSS_32_NODES.iter()
        .map(|x| 0.5 * (s_max - s_min) * x + 0.5 * (s_min + s_max))
        .zip(GAUSS_32_WEIGHTS.iter())
        .map(|(s, w)| {
            let sp = partial_spectrum(n, a, eta, s);
            0.5 * (s_max - s_min) * w * sp
        })
        .sum();
    2.0 * rate
}

pub fn probability(ell: FourVector, k: FourVector, a: f64, dt: f64) -> Option<f64> {
    let eta = k * ell;
    let n = (2.0  / eta).ceil() as i32;
    let f = if a < 0.02 {
        0.0
    } else {
        partial_rate(n, a, eta)
    };
    assert!(f.is_finite());
    Some(ALPHA_FINE * f * dt / (COMPTON_TIME * ell[0]))
}

pub fn generate<R: Rng>(ell: FourVector, k: FourVector, a: f64, rng: &mut R) -> (i32, FourVector) {
    let eta: f64 = k * ell;
    let j = (2.0 / eta).ceil();
    let n = j as i32;

    let sn = 2.0 * j * eta;
    let s_min = 0.5 - (0.25 - 1.0 / sn).sqrt();

    // Approximate maximum value of the probability density:
    let max = 1.1 * partial_spectrum(n, a, eta, 0.5);

    // Rejection sampling for s = k.p / k.ell
    let s = loop {
        let s = s_min + (1.0 - 2.0 * s_min) * rng.gen::<f64>();
        let u = rng.gen::<f64>();
        let f = partial_spectrum(n, a, eta, s);
        if u <= f / max {
            break s;
        }
    };

    // Scattering momentum (/m) and angles in zero momentum frame
    // if ell_perp = 0, (q_perp/m)^2 = 2 n eta s (1-s) - 1
    let e_zmf = (0.5 * j * eta).sqrt();
    let p_zmf = (0.5 * j * eta - 1.0).sqrt();
    let cos_theta_zmf = (1.0 - 2.0 * s) * e_zmf / p_zmf;
    let cphi_zmf = 2.0 * consts::PI * rng.gen::<f64>();

    assert!(cos_theta_zmf <= 1.0);
    assert!(cos_theta_zmf >= -1.0);

    // Four-velocity of ZMF (normalized)
    let u_zmf: FourVector = (ell + j * k) / (ell + j * k).norm_sqr().sqrt();

    // Unit vectors pointed parallel to gamma-ray momentum in ZMF
    // and perpendicular to it
    let along = -ThreeVector::from(ell.boost_by(u_zmf)).normalize();
    let perp = along.orthogonal().rotate_around(along, cphi_zmf);

    // Construct positron momentum and transform back to lab frame
    let q: ThreeVector = p_zmf * (cos_theta_zmf * along + (1.0 - cos_theta_zmf.powi(2)).sqrt() * perp);
    let q = FourVector::lightlike(q[0], q[1], q[2]).with_sqr(1.0);
    let q = q.boost_by(u_zmf.reverse());

    (n, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_order_rates() {
        let max_error = 1.0e-6;

        // n, a, eta, target
        let pts = [
            (21, 0.01, 0.1, 4.2971515895264524404e-97),
            (21, 0.1,  0.1, 4.2971515895264524404e-55),
            (11, 0.01, 0.2, 3.9354540997449447469e-49),
            (11, 0.1,  0.2, 3.9354540997449447469e-27),
        ];

        for (n, a, eta, target) in &pts {
            let result = partial_rate(*n, *a, *eta);
            let error = (target - result).abs() / target;
            println!("n = {}, a = {}, eta = {}, result = {:.6e}, error = {:.6e}", n, a, eta, result, error);
            assert!(error < max_error);
        }
    }
}