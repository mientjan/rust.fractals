use std::time::{Duration, Instant};
use crate::fractal::FractalType;

pub struct AutoPlay {
    enabled: bool,
    location_index: usize,
    location_timer: Instant,
    fractal_timer: Instant,
    zoom_start: f64,
    center_start: (f64, f64),
    location_duration: Duration,
    fractal_duration: Duration,
    /// True when currently targeting a deep-zoom location
    in_deep_zoom: bool,
    /// High-precision string coordinates for current deep-zoom target
    deep_target_re: String,
    deep_target_im: String,
}

impl AutoPlay {
    pub fn new() -> Self {
        let now = Instant::now();
        AutoPlay {
            enabled: true,
            location_index: 0,
            location_timer: now,
            fractal_timer: now,
            zoom_start: 1.0,
            center_start: (-0.5, 0.0),
            location_duration: Duration::from_secs(18),
            fractal_duration: Duration::from_secs(100),
            in_deep_zoom: false,
            deep_target_re: String::new(),
            deep_target_im: String::new(),
        }
    }

    pub fn toggle(&mut self, current_zoom: f64, current_re: f64, current_im: f64) {
        self.enabled = !self.enabled;
        if self.enabled {
            let now = Instant::now();
            self.location_timer = now;
            self.fractal_timer = now;
            self.zoom_start = current_zoom;
            self.center_start = (current_re, current_im);
            self.in_deep_zoom = false;
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the high-precision center strings if currently in a deep zoom.
    pub fn deep_zoom_center(&self) -> Option<(&str, &str)> {
        if self.in_deep_zoom && !self.deep_target_re.is_empty() {
            Some((&self.deep_target_re, &self.deep_target_im))
        } else {
            None
        }
    }

    /// Returns true if fractal type changed.
    pub fn update(
        &mut self,
        fractal: &mut FractalType,
        center_re: &mut f64,
        center_im: &mut f64,
        zoom: &mut f64,
    ) -> bool {
        if !self.enabled {
            return false;
        }

        let now = Instant::now();
        let mut fractal_changed = false;

        // Cycle fractal type after visiting all locations
        if now.duration_since(self.fractal_timer) >= self.fractal_duration {
            *fractal = fractal.next();
            let (re, im, z) = fractal.default_view();
            *center_re = re;
            *center_im = im;
            *zoom = z;
            self.zoom_start = z;
            self.center_start = (re, im);
            self.location_index = 0;
            self.fractal_timer = now;
            self.location_timer = now;
            self.in_deep_zoom = false;
            fractal_changed = true;
        }

        let locations = fractal.interesting_locations();
        let deep_locations = fractal.interesting_locations_deep();
        let total_locations = locations.len() + deep_locations.len();

        // Advance to next interesting location
        if now.duration_since(self.location_timer) >= self.location_duration {
            self.center_start = (*center_re, *center_im);
            self.zoom_start = *zoom;
            self.location_index = (self.location_index + 1) % total_locations;
            self.location_timer = now;
        }

        if self.location_index < locations.len() {
            // Standard f64 location
            self.in_deep_zoom = false;
            let (target_re, target_im, target_zoom) = locations[self.location_index];
            let elapsed = now.duration_since(self.location_timer).as_secs_f64();
            let duration = self.location_duration.as_secs_f64();
            let t = ease_in_out((elapsed / duration).min(1.0));

            *center_re = self.center_start.0 + (target_re - self.center_start.0) * t;
            *center_im = self.center_start.1 + (target_im - self.center_start.1) * t;
            let log_start = self.zoom_start.ln();
            let log_target = target_zoom.ln();
            *zoom = (log_start + (log_target - log_start) * t).exp();
        } else {
            // Deep zoom location (string-based coordinates)
            let deep_idx = self.location_index - locations.len();
            let (target_re_str, target_im_str, target_zoom) = deep_locations[deep_idx];

            self.in_deep_zoom = true;
            self.deep_target_re = target_re_str.to_string();
            self.deep_target_im = target_im_str.to_string();

            // Parse target for f64 interpolation (approximate, but close enough for display)
            let target_re: f64 = target_re_str.parse().unwrap_or(0.0);
            let target_im: f64 = target_im_str.parse().unwrap_or(0.0);

            let elapsed = now.duration_since(self.location_timer).as_secs_f64();
            let duration = self.location_duration.as_secs_f64();
            let t = ease_in_out((elapsed / duration).min(1.0));

            *center_re = self.center_start.0 + (target_re - self.center_start.0) * t;
            *center_im = self.center_start.1 + (target_im - self.center_start.1) * t;
            let log_start = self.zoom_start.ln();
            let log_target = target_zoom.ln();
            *zoom = (log_start + (log_target - log_start) * t).exp();
        }

        fractal_changed
    }
}

#[inline]
fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}
