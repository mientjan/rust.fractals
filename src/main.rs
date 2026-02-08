mod autoplay;
mod fractal;
mod menu;
mod objects;
mod utils;

use autoplay::AutoPlay;
use fractal::FractalType;
use menu::Menu;
use objects::Complex;
use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use menu::render_hud;
use utils::math::{burning_ship, julia, mandelbrot_de, newton, tricorn, ColorPalette};
use utils::perturbation::{compute_reference_orbit, mandelbrot_perturbation, GLITCH_SENTINEL};
use utils::view::create_texture;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BASE_ITER: u32 = 256;
const PERTURBATION_ZOOM_THRESHOLD: f64 = 1e13;
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Julia set constant (c = -0.8 + 0.156i)
const JULIA_C_RE: f64 = -0.8;
const JULIA_C_IM: f64 = 0.156;

/// Scale iterations with zoom so boundary detail stays visible at deep zooms.
#[inline]
fn max_iter_for_zoom(zoom: f64) -> u32 {
    let extra = (zoom.ln().max(0.0) * 80.0) as u32;
    (BASE_ITER + extra).min(4000)
}

fn generate_fractal(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    zoom: f64,
    center_re: f64,
    center_im: f64,
    fractal_type: FractalType,
    palette: &ColorPalette,
) {
    let w = width as f64;
    let h = height as f64;
    let aspect = w / h;
    let max_iter = max_iter_for_zoom(zoom);
    let pixel_size = 4.0 / (zoom * h); // Size of one pixel in complex plane

    let row_bytes = width as usize * 3;
    pixels
        .par_chunks_mut(row_bytes)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..width as usize {
                let re = (x as f64 / w - 0.5) * 4.0 * aspect / zoom + center_re;
                let im = (y as f64 / h - 0.5) * 4.0 / zoom + center_im;

                let t = match fractal_type {
                    FractalType::Mandelbrot => {
                        mandelbrot_de(Complex::new(re, im), max_iter, pixel_size)
                    }
                    FractalType::Julia => {
                        julia(Complex::new(re, im), Complex::new(JULIA_C_RE, JULIA_C_IM), max_iter)
                    }
                    FractalType::BurningShip => burning_ship(Complex::new(re, im), max_iter),
                    FractalType::Tricorn => tricorn(Complex::new(re, im), max_iter),
                    FractalType::Newton => newton(Complex::new(re, im), max_iter),
                };

                let [r, g, b] = if t == 0.0 { [0, 0, 0] } else { palette.lookup(t) };
                let offset = x * 3;
                row[offset] = r;
                row[offset + 1] = g;
                row[offset + 2] = b;
            }
        });
}

/// Compute precision bits needed for a given zoom level.
/// Roughly 3.32 bits per power of 10, plus headroom.
#[inline]
fn precision_for_zoom(zoom: f64) -> u32 {
    let bits = (zoom.log10() * 3.32 + 64.0) as u32;
    bits.max(64).min(1024)
}

fn generate_fractal_perturbation(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    zoom: f64,
    center_re_str: &str,
    center_im_str: &str,
    palette: &ColorPalette,
) {
    let w = width as f64;
    let h = height as f64;
    let aspect = w / h;
    let max_iter = max_iter_for_zoom(zoom);
    let precision = precision_for_zoom(zoom);

    // Compute reference orbit at full precision (single-threaded)
    let ref_orbit = compute_reference_orbit(center_re_str, center_im_str, precision, max_iter);

    let row_bytes = width as usize * 3;
    pixels
        .par_chunks_mut(row_bytes)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..width as usize {
                // Delta from center (fits in f64 since pixels are close together)
                let delta_re = (x as f64 / w - 0.5) * 4.0 * aspect / zoom;
                let delta_im = (y as f64 / h - 0.5) * 4.0 / zoom;

                let t = mandelbrot_perturbation(&ref_orbit, delta_re, delta_im, max_iter);

                let [r, g, b] = if t == GLITCH_SENTINEL {
                    // Glitched pixel: render with a visible marker (magenta)
                    [255, 0, 255]
                } else if t == 0.0 {
                    [0, 0, 0]
                } else {
                    palette.lookup(t)
                };
                let offset = x * 3;
                row[offset] = r;
                row[offset + 1] = g;
                row[offset + 2] = b;
            }
        });
}

/// High-precision center coordinates stored as strings.
struct HighPrecCenter {
    re: String,
    im: String,
}

impl HighPrecCenter {
    fn from_f64(re: f64, im: f64) -> Self {
        HighPrecCenter {
            re: format!("{:.20}", re),
            im: format!("{:.20}", im),
        }
    }

    fn update_from_f64(&mut self, re: f64, im: f64) {
        self.re = format!("{:.20}", re);
        self.im = format!("{:.20}", im);
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video = sdl_context.video().expect("Failed to initialize video subsystem");
    let ttf_context = sdl2::ttf::init().expect("Failed to initialize SDL2_ttf");

    let window = video
        .window(&format!("Fractal Viewer v{} - Mandelbrot", VERSION), WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to create canvas");
    let texture_creator = canvas.texture_creator();
    let mut texture = create_texture(&texture_creator, WIDTH, HEIGHT);

    let font = ttf_context
        .load_font("/Library/Fonts/Arial Unicode.ttf", 15)
        .expect("Failed to load font");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    let mut current_fractal = FractalType::Mandelbrot;
    let mut zoom = 1.0_f64;
    let mut center_re = -0.5;
    let mut center_im = 0.0;
    let zoom_speed = 1.02;
    let pan_speed = 0.05;

    let palette = ColorPalette::new();
    let mut menu = Menu::new();
    let mut autoplay = AutoPlay::new();
    let mut hi_center = HighPrecCenter::from_f64(center_re, center_im);

    'running: loop {
        let mut title_changed = false;

        // Event handling (one-shot keys)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                Event::KeyDown { keycode: Some(Keycode::Tab), .. } => menu.toggle(),
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    autoplay.toggle(zoom, center_re, center_im);
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let (re, im, z) = current_fractal.default_view();
                    center_re = re;
                    center_im = im;
                    zoom = z;
                }

                // Number keys to switch fractal
                Event::KeyDown { keycode: Some(k), .. } => {
                    let idx = match k {
                        Keycode::Num1 => Some(0),
                        Keycode::Num2 => Some(1),
                        Keycode::Num3 => Some(2),
                        Keycode::Num4 => Some(3),
                        Keycode::Num5 => Some(4),
                        _ => None,
                    };
                    if let Some(i) = idx {
                        if let Some(ft) = FractalType::from_index(i) {
                            current_fractal = ft;
                            let (re, im, z) = ft.default_view();
                            center_re = re;
                            center_im = im;
                            zoom = z;
                            title_changed = true;
                        }
                    }
                }
                _ => {}
            }
        }

        // Auto-play updates
        if autoplay.is_enabled() {
            if autoplay.update(&mut current_fractal, &mut center_re, &mut center_im, &mut zoom) {
                title_changed = true;
            }
        } else {
            // Manual keyboard navigation (held keys)
            let keys: Vec<Keycode> = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            for key in &keys {
                match *key {
                    Keycode::Up | Keycode::W => center_im -= pan_speed / zoom,
                    Keycode::Down | Keycode::S => center_im += pan_speed / zoom,
                    Keycode::Left | Keycode::A => center_re -= pan_speed / zoom,
                    Keycode::Right | Keycode::D => center_re += pan_speed / zoom,
                    Keycode::E | Keycode::Plus | Keycode::Equals => zoom *= zoom_speed,
                    Keycode::Q | Keycode::Minus => zoom /= zoom_speed,
                    _ => {}
                }
            }
        }

        if title_changed {
            canvas
                .window_mut()
                .set_title(&format!("Fractal Viewer v{} - {}", VERSION, current_fractal.name()))
                .ok();
        }

        // Keep high-precision center in sync
        if let Some((deep_re, deep_im)) = autoplay.deep_zoom_center() {
            hi_center.re = deep_re.to_string();
            hi_center.im = deep_im.to_string();
        } else {
            hi_center.update_from_f64(center_re, center_im);
        }

        // Render fractal — switch to perturbation for deep Mandelbrot zoom
        let use_perturbation = current_fractal == FractalType::Mandelbrot
            && zoom >= PERTURBATION_ZOOM_THRESHOLD;

        texture
            .with_lock(None, |pixels, _pitch| {
                if use_perturbation {
                    generate_fractal_perturbation(
                        pixels, WIDTH, HEIGHT, zoom,
                        &hi_center.re, &hi_center.im, &palette,
                    );
                } else {
                    generate_fractal(
                        pixels, WIDTH, HEIGHT, zoom, center_re, center_im,
                        current_fractal, &palette,
                    );
                }
            })
            .expect("Failed to lock texture");

        canvas.copy(&texture, None, None).expect("Failed to copy texture");

        // Draw menu overlay on top
        menu.render(&mut canvas, &font, current_fractal, autoplay.is_enabled())
            .ok();

        // Always-visible HUD: zoom + coordinates
        render_hud(&mut canvas, &font, zoom, center_re, center_im, WIDTH).ok();

        canvas.present();
    }
}
