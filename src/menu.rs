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
