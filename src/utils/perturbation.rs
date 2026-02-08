use crate::objects::BigComplex;
use rug::Float;

/// A pre-computed reference orbit stored as f64 pairs for fast delta iteration.
pub struct ReferenceOrbit {
    /// Z(n) values as (re, im) in f64
    pub points: Vec<(f64, f64)>,
    /// Number of iterations before the reference escaped (or max_iter)
    #[allow(dead_code)]
    pub escape_iter: u32,
}

/// Compute the reference orbit at arbitrary precision.
/// Center coordinates are given as decimal strings for full precision.
pub fn compute_reference_orbit(
    center_re: &str,
    center_im: &str,
    precision_bits: u32,
    max_iter: u32,
) -> ReferenceOrbit {
    let c = BigComplex::from_str(center_re, center_im, precision_bits);
    let mut z = BigComplex::new(
        Float::with_val(precision_bits, 0.0),
        Float::with_val(precision_bits, 0.0),
    );

    let mut points = Vec::with_capacity(max_iter as usize);
    let mut escape_iter = max_iter;

    for i in 0..max_iter {
        let (re, im) = z.to_f64_pair();
        points.push((re, im));

        if re * re + im * im > 1e6 {
            escape_iter = i;
            break;
        }

        z.square_add_assign(&c);
    }

    ReferenceOrbit {
        points,
        escape_iter,
    }
}

/// Sentinel value indicating a glitched pixel that needs re-rendering.
pub const GLITCH_SENTINEL: f64 = -1.0;

/// Mandelbrot iteration using perturbation theory.
/// `ref_orbit` is the pre-computed reference orbit.
/// `delta_re`, `delta_im` is the offset from the reference center (in f64).
/// Returns smooth iteration count in [0, 1], 0.0 for interior, or GLITCH_SENTINEL for glitched pixels.
#[inline]
pub fn mandelbrot_perturbation(
    ref_orbit: &ReferenceOrbit,
    delta_re: f64,
    delta_im: f64,
    max_iter: u32,
) -> f64 {
    let d0_re = delta_re;
    let d0_im = delta_im;
    let mut d_re = d0_re;
    let mut d_im = d0_im;

    let orbit_len = ref_orbit.points.len();
    let iters = orbit_len.min(max_iter as usize);

    for i in 0..iters {
        let (z_re, z_im) = ref_orbit.points[i];

        // Full z = Z(n) + delta(n)
        let full_re = z_re + d_re;
        let full_im = z_im + d_im;
        let full_r2 = full_re * full_re + full_im * full_im;

        // Escape check on full z
        if full_r2 > 256.0 {
            let log_zn = full_r2.ln() / 2.0;
            let nu = log_zn.log2();
            return (i as f64 + 1.0 - nu) / max_iter as f64;
        }

        // Glitch detection: if delta is too large relative to Z, perturbation is unreliable
        let z_r2 = z_re * z_re + z_im * z_im;
        let d_r2 = d_re * d_re + d_im * d_im;
        if z_r2 > 0.0 && d_r2 > z_r2 * 1000.0 {
            return GLITCH_SENTINEL;
        }

        // Delta iteration: delta(n+1) = 2*Z(n)*delta(n) + delta(n)^2 + delta(0)
        // (2*Z + delta) * delta + delta0
        let two_z_plus_d_re = 2.0 * z_re + d_re;
        let two_z_plus_d_im = 2.0 * z_im + d_im;
        // Complex multiply: (2Z+d) * d
        let new_d_re = two_z_plus_d_re * d_re - two_z_plus_d_im * d_im + d0_re;
        let new_d_im = two_z_plus_d_re * d_im + two_z_plus_d_im * d_re + d0_im;
        d_re = new_d_re;
        d_im = new_d_im;
    }

    0.0
}
