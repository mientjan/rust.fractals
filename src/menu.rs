use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

use crate::fractal::{FractalType, ALL_FRACTALS};

pub struct Menu {
    visible: bool,
}

impl Menu {
    pub fn new() -> Self {
        Menu { visible: false }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    #[allow(dead_code)]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        current: FractalType,
        autoplay_on: bool,
    ) -> Result<(), String> {
        if !self.visible {
            return Ok(());
        }

        // Semi-transparent background panel
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
        let panel = Rect::new(20, 20, 320, 300);
        canvas.fill_rect(panel)?;
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.draw_rect(panel)?;

        let mut y = 30;
        let x = 35;

        render_line(canvas, font, "FRACTAL VIEWER", x, y, Color::RGB(255, 255, 255))?;
        y += 28;

        // Fractal list
        for (i, ftype) in ALL_FRACTALS.iter().enumerate() {
            let selected = *ftype == current;
            let col = if selected {
                Color::RGB(100, 220, 255)
            } else {
                Color::RGB(180, 180, 180)
            };
            let prefix = if selected { "> " } else { "  " };
            let label = format!("{}{}: {}", prefix, i + 1, ftype.name());
            render_line(canvas, font, &label, x, y, col)?;
            y += 22;
        }

        y += 10;
        let ap_col = if autoplay_on {
            Color::RGB(100, 255, 100)
        } else {
            Color::RGB(180, 180, 180)
        };
        let ap_text = format!("Auto-play: {}", if autoplay_on { "ON" } else { "OFF" });
        render_line(canvas, font, &ap_text, x, y, ap_col)?;

        y += 28;
        render_line(canvas, font, "Controls:", x, y, Color::RGB(140, 140, 140))?;
        y += 20;
        for line in &[
            "1-5  Select fractal",
            "WASD Pan",
            "E/Q  Zoom in/out",
            "Space  Toggle auto-play",
            "Tab  Toggle this menu",
            "R  Reset view",
            "Esc  Quit",
        ] {
            render_line(canvas, font, line, x + 10, y, Color::RGB(160, 160, 160))?;
            y += 18;
        }

        Ok(())
    }
}

/// Always-visible HUD: zoom level + coordinates, top-right corner.
pub fn render_hud(
    canvas: &mut Canvas<Window>,
    font: &Font,
    zoom: f64,
    center_re: f64,
    center_im: f64,
    canvas_width: u32,
) -> Result<(), String> {
    let zoom_text = if zoom >= 1e15 {
        format!("Zoom: {:.2e}", zoom)
    } else if zoom >= 1000.0 {
        format!("Zoom: {:.0}", zoom)
    } else {
        format!("Zoom: {:.2}", zoom)
    };

    let re_text = format!("Re: {:.15}", center_re);
    let im_text = format!("Im: {:.15}", center_im);

    let margin = 10;
    let col = Color::RGBA(220, 220, 220, 200);

    // Background panel
    let panel_w = 280u32;
    let panel_h = 56u32;
    let panel_x = canvas_width as i32 - panel_w as i32 - margin;
    let panel_y = margin;

    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 150));
    canvas.fill_rect(Rect::new(panel_x, panel_y, panel_w, panel_h))?;

    let x = panel_x + 6;
    render_line(canvas, font, &zoom_text, x, panel_y + 4, col)?;
    render_line(canvas, font, &re_text, x, panel_y + 20, col)?;
    render_line(canvas, font, &im_text, x, panel_y + 36, col)?;

    Ok(())
}

fn render_line(
    canvas: &mut Canvas<Window>,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let tc = canvas.texture_creator();
    let texture = tc
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let w = surface.width();
    let h = surface.height();
    canvas.copy(&texture, None, Some(Rect::new(x, y, w, h)))?;
    Ok(())
}
