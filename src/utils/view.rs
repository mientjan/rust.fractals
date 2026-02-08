use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub fn create_texture(texture_creator: &TextureCreator<WindowContext>, width: u32, height: u32) -> Texture<'_> {
    texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .expect("Failed to create texture")
}
