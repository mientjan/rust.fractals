use crate::objects::Complex;

/// Fast check if point is inside the main cardioid or period-2 bulb.
/// Skips iteration entirely for ~30% of Mandelbrot viewport.
#[inline]
fn in_cardioid_or_bulb(re: f64, im: f64) -> bool {
    // Main cardioid check
    let im2 = im * im;
    let q = (re - 0.25) * (re - 0.25) + im2;
    if q * (q + (re - 0.25)) <= 0.25 * im2 {
        return true;
    }
    // Period-2 bulb check
    if (re + 1.0) * (re + 1.0) + im2 <= 0.0625 {
        return true;
    }
    false
}

#[allow(dead_code)]
#[inline]
pub fn mandelbrot(c: Complex, max_iter: u32) -> f64 {
    // Skip known interior regions
    if in_cardioid_or_bulb(c.re, c.im) {
        return 0.0;
    }

    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;

    // Periodicity checking: remember an old z and compare
    let mut old_re = 0.0_f64;
    let mut old_im = 0.0_f64;
    let mut period = 0u32;
    let mut check = 8u32; // Check interval, doubles over time

    let mut i = 0u32;
    while i < max_iter {
        let re2 = z_re * z_re;
        let im2 = z_im * z_im;
        if re2 + im2 > 256.0 {
            // Smooth iteration count
            let log_zn = (re2 + im2).ln() / 2.0;
            let nu = log_zn.log2();
            return (i as f64 + 1.0 - nu) / max_iter as f64;
        }
        let new_im = 2.0 * z_re * z_im + c.im;
        z_re = re2 - im2 + c.re;
        z_im = new_im;

        // Periodicity check: if z returns to a previous value, it's in the set
        if z_re == old_re && z_im == old_im {
            return 0.0;
        }
        period += 1;
        if period >= check {
            old_re = z_re;
            old_im = z_im;
            period = 0;
            check = check.saturating_mul(2); // Exponential backoff
        }

        i += 1;
    }
    0.0
}

#[inline]
pub fn julia(z0: Complex, c: Complex, max_iter: u32) -> f64 {
    let mut z_re = z0.re;
    let mut z_im = z0.im;

    let mut old_re = z_re;
    let mut old_im = z_im;
    let mut period = 0u32;
    let mut check = 8u32;

    let mut i = 0u32;
    while i < max_iter {
        let re2 = z_re * z_re;
        let im2 = z_im * z_im;
        if re2 + im2 > 256.0 {
            let log_zn = (re2 + im2).ln() / 2.0;
            let nu = log_zn.log2();
            return (i as f64 + 1.0 - nu) / max_iter as f64;
        }
        let new_im = 2.0 * z_re * z_im + c.im;
        z_re = re2 - im2 + c.re;
        z_im = new_im;

        if z_re == old_re && z_im == old_im {
            return 0.0;
        }
        period += 1;
        if period >= check {
            old_re = z_re;
            old_im = z_im;
            period = 0;
            check = check.saturating_mul(2);
        }

        i += 1;
    }
    0.0
}

#[inline]
pub fn burning_ship(c: Complex, max_iter: u32) -> f64 {
    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;

    let mut old_re = 0.0_f64;
    let mut old_im = 0.0_f64;
    let mut period = 0u32;
    let mut check = 8u32;

    let mut i = 0u32;
    while i < max_iter {
        let re2 = z_re * z_re;
        let im2 = z_im * z_im;
        if re2 + im2 > 256.0 {
            let log_zn = (re2 + im2).ln() / 2.0;
            let nu = log_zn.log2();
            return (i as f64 + 1.0 - nu) / max_iter as f64;
        }
        let new_im = 2.0 * z_re.abs() * z_im.abs() + c.im;
        z_re = re2 - im2 + c.re;
        z_im = new_im;

        if z_re == old_re && z_im == old_im {
            return 0.0;
        }
        period += 1;
        if period >= check {
            old_re = z_re;
            old_im = z_im;
            period = 0;
            check = check.saturating_mul(2);
        }

        i += 1;
    }
    0.0
}

#[inline]
pub fn tricorn(c: Complex, max_iter: u32) -> f64 {
    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;

    let mut old_re = 0.0_f64;
    let mut old_im = 0.0_f64;
    let mut period = 0u32;
    let mut check = 8u32;

    let mut i = 0u32;
    while i < max_iter {
        let re2 = z_re * z_re;
        let im2 = z_im * z_im;
        if re2 + im2 > 256.0 {
            let log_zn = (re2 + im2).ln() / 2.0;
            let nu = log_zn.log2();
            return (i as f64 + 1.0 - nu) / max_iter as f64;
        }
        // Conjugate: flip im sign before squaring
        let new_im = -2.0 * z_re * z_im + c.im;
        z_re = re2 - im2 + c.re;
        z_im = new_im;

        if z_re == old_re && z_im == old_im {
            return 0.0;
        }
        period += 1;
        if period >= check {
            old_re = z_re;
            old_im = z_im;
            period = 0;
            check = check.saturating_mul(2);
        }

        i += 1;
    }
    0.0
}

#[inline]
pub fn newton(z0: Complex, max_iter: u32) -> f64 {
    let mut z = z0;
    let one = Complex::new(1.0, 0.0);
    let three = Complex::new(3.0, 0.0);
    let tolerance = 1e-6;

    // The three roots of z^3 - 1 = 0
    let roots = [
        Complex::new(1.0, 0.0),
        Complex::new(-0.5, 0.866_025_403_784_438_6),
        Complex::new(-0.5, -0.866_025_403_784_438_6),
    ];

    let mut i = 0u32;
    while i < max_iter {
        let z2 = z * z;
        let z3 = z2 * z;
        let denom = z2 * three;

        if denom.norm_sq() < tolerance {
            break;
        }

        z = z - (z3 - one) / denom;
        i += 1;

        // Check convergence to any root
        for (root_idx, root) in roots.iter().enumerate() {
            let dr = z.re - root.re;
            let di = z.im - root.im;
            if dr * dr + di * di < tolerance {
                let shade = 1.0 - (i as f64 / max_iter as f64);
                return (root_idx as f64 / 3.0) + shade * 0.3;
            }
        }
    }
    0.0
}

/// Mandelbrot with distance estimation.
/// Returns (smooth_t, distance) where distance is the estimated distance
/// from the point to the Mandelbrot set boundary.
#[inline]
pub fn mandelbrot_de(c: Complex, max_iter: u32, pixel_size: f64) -> f64 {
    if in_cardioid_or_bulb(c.re, c.im) {
        return 0.0;
    }

    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;
    // Derivative dz/dc, starts at 0
    let mut dz_re = 0.0_f64;
    let mut dz_im = 0.0_f64;

    let mut old_re = 0.0_f64;
    let mut old_im = 0.0_f64;
    let mut period = 0u32;
    let mut check = 8u32;

    let mut i = 0u32;
    while i < max_iter {
        let re2 = z_re * z_re;
        let im2 = z_im * z_im;
        let r2 = re2 + im2;

        if r2 > 256.0 {
            // Distance estimate: d = 2 * |z| * ln(|z|) / |dz/dc|
            let z_abs = r2.sqrt();
            let dz_abs = (dz_re * dz_re + dz_im * dz_im).sqrt();
            let dist = if dz_abs > 0.0 {
                2.0 * z_abs * z_abs.ln() / dz_abs
            } else {
                f64::MAX
            };

            // If point is far from boundary relative to pixel, use fast path
            if dist > pixel_size * 5.0 {
                // Far exterior: return a value based on iteration only (skip smooth calc)
                return i as f64 / max_iter as f64;
            }

            let log_zn = r2.ln() / 2.0;
            let nu = log_zn.log2();
            return (i as f64 + 1.0 - nu) / max_iter as f64;
        }

        // Update derivative: dz' = 2*z*dz + 1
        let new_dz_re = 2.0 * (z_re * dz_re - z_im * dz_im) + 1.0;
        let new_dz_im = 2.0 * (z_re * dz_im + z_im * dz_re);
        dz_re = new_dz_re;
        dz_im = new_dz_im;

        let new_im = 2.0 * z_re * z_im + c.im;
        z_re = re2 - im2 + c.re;
        z_im = new_im;

        if z_re == old_re && z_im == old_im {
            return 0.0;
        }
        period += 1;
        if period >= check {
            old_re = z_re;
            old_im = z_im;
            period = 0;
            check = check.saturating_mul(2);
        }

        i += 1;
    }
    0.0
}

const PALETTE_SIZE: usize = 4096;

/// Pre-computed color lookup table. Eliminates trig from the hot loop.
pub struct ColorPalette {
    table: [[u8; 3]; PALETTE_SIZE],
}

impl ColorPalette {
    pub fn new() -> Self {
        let mut table = [[0u8; 3]; PALETTE_SIZE];
        let tau = std::f64::consts::TAU;
        let a = 0.5;
        let b = 0.5;
        let d = [0.0, 0.10, 0.20];

        for i in 0..PALETTE_SIZE {
            let t = i as f64 / PALETTE_SIZE as f64;
            let r = (b * (tau * (t + d[0])).cos() + a).clamp(0.0, 1.0);
            let g = (b * (tau * (t + d[1])).cos() + a).clamp(0.0, 1.0);
            let bl = (b * (tau * (t + d[2])).cos() + a).clamp(0.0, 1.0);
            table[i] = [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * bl) as u8];
        }

        ColorPalette { table }
    }

    #[inline]
    pub fn lookup(&self, t: f64) -> [u8; 3] {
        let idx = ((t * PALETTE_SIZE as f64) as usize).min(PALETTE_SIZE - 1);
        self.table[idx]
    }
}

#[allow(dead_code)]
#[inline]
pub fn color(t: f64) -> [u8; 3] {
    let tau = std::f64::consts::TAU;
    let a = 0.5;
    let b = 0.5;
    let d = [0.0, 0.10, 0.20];

    let r = (b * (tau * (t + d[0])).cos() + a).clamp(0.0, 1.0);
    let g = (b * (tau * (t + d[1])).cos() + a).clamp(0.0, 1.0);
    let bl = (b * (tau * (t + d[2])).cos() + a).clamp(0.0, 1.0);

    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * bl) as u8]
}
