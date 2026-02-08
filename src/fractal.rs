#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
    Newton,
}

pub const ALL_FRACTALS: [FractalType; 5] = [
    FractalType::Mandelbrot,
    FractalType::Julia,
    FractalType::BurningShip,
    FractalType::Tricorn,
    FractalType::Newton,
];

impl FractalType {
    pub fn name(&self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot",
            FractalType::Julia => "Julia",
            FractalType::BurningShip => "Burning Ship",
            FractalType::Tricorn => "Tricorn",
            FractalType::Newton => "Newton (z^3-1)",
        }
    }

    pub fn default_view(&self) -> (f64, f64, f64) {
        // (center_re, center_im, zoom)
        match self {
            FractalType::Mandelbrot => (-0.5, 0.0, 1.0),
            FractalType::Julia => (0.0, 0.0, 1.0),
            FractalType::BurningShip => (-0.4, -0.5, 1.0),
            FractalType::Tricorn => (0.0, 0.0, 1.0),
            FractalType::Newton => (0.0, 0.0, 1.5),
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        ALL_FRACTALS.get(index).copied()
    }

    pub fn index(&self) -> usize {
        ALL_FRACTALS.iter().position(|f| f == self).unwrap()
    }

    pub fn next(&self) -> Self {
        let i = (self.index() + 1) % ALL_FRACTALS.len();
        ALL_FRACTALS[i]
    }

    /// Deep zoom locations for perturbation theory rendering.
    /// Returns (center_re_str, center_im_str, target_zoom) with high-precision string coordinates.
    /// Only Mandelbrot has deep zoom points; others return empty.
    pub fn interesting_locations_deep(&self) -> &'static [(&'static str, &'static str, f64)] {
        match self {
            FractalType::Mandelbrot => &[
                // Seahorse valley — deep spiral
                (
                    "-0.7436438885706",
                    "0.1318259043124",
                    1e14,
                ),
                // Mini-brot in elephant valley
                (
                    "-1.7497591451303",
                    "0.0000000799798",
                    1e15,
                ),
                // Deep seahorse spiral
                (
                    "-0.74364388857060744",
                    "0.13182590431247862",
                    1e17,
                ),
                // Double spiral — antenna
                (
                    "-1.749759145130375046",
                    "0.000000079979893876",
                    1e20,
                ),
            ],
            _ => &[],
        }
    }

    pub fn interesting_locations(&self) -> &'static [(f64, f64, f64)] {
        // (center_re, center_im, target_zoom)
        match self {
            // Mandelbrot: verified boundary coordinates
            FractalType::Mandelbrot => &[
                (-0.7436439, 0.1318259, 500.0),                // Double spiral in seahorse valley
                (-1.74995768, 0.00000006, 1_500.0),             // Mini-brot on the real axis
                (-0.10109636, 0.95628651, 300.0),               // Spiral at top of main cardioid
                (-0.7436439, 0.1318259, 5_000.0),               // Deeper into seahorse spiral
                (0.25000, 0.00000, 500.0),                      // Cusp of main cardioid
                (-0.74364085, 0.13182733, 50_000.0),            // Deep seahorse spiral
            ],
            // Julia (c = -0.8 + 0.156i): boundary spirals
            FractalType::Julia => &[
                (-0.16, 0.82, 80.0),                            // Boundary filament
                (0.12, -0.58, 150.0),                           // Lower spiral arm
                (0.38, 0.18, 300.0),                            // Right side detail
                (-0.42, -0.34, 500.0),                          // Left spiral
                (0.0, 0.0, 20.0),                               // Center zoom
            ],
            // Burning Ship: distinctive features
            FractalType::BurningShip => &[
                (-1.755, -0.032, 50.0),                         // Ship antenna overview
                (-1.7610, -0.0280, 500.0),                      // Antenna detail
                (-0.515, -0.520, 50.0),                         // Mini-ship
                (-1.755, -0.032, 2_000.0),                      // Deep antenna
                (-0.45, -0.55, 200.0),                          // Ship hull detail
            ],
            // Tricorn: cusp and boundary detail
            FractalType::Tricorn => &[
                (-1.0, 0.0, 50.0),                              // Left cusp
                (0.5, 0.866, 50.0),                              // Upper-right cusp
                (0.5, -0.866, 50.0),                             // Lower-right cusp
                (-0.5, 0.5, 200.0),                              // Boundary detail
                (-1.0, 0.0, 500.0),                              // Deeper into left cusp
            ],
            // Newton (z^3-1): basin boundaries
            FractalType::Newton => &[
                (0.0, 0.0, 30.0),                               // Central junction zoom
                (0.5, 0.28868, 100.0),                           // Boundary roots 1 & 2
                (-0.5, 0.28868, 100.0),                          // Boundary roots 2 & 3
                (0.0, -0.57735, 100.0),                          // Boundary roots 1 & 3
                (0.0, 0.0, 500.0),                               // Deep central zoom
            ],
        }
    }
}
