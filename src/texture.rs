use crate::color::Color;
use image::RgbImage;
use std::path::Path;

#[derive(Clone)]
pub struct Texture {
    pub image: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let img = image::open(&Path::new(path)).expect("Failed to load texture image");
        let rgb_img = img.to_rgb8();
        let width = rgb_img.width();
        let height = rgb_img.height();

        Texture {
            image: rgb_img,
            width,
            height,
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Calculate the pixel coordinates, with v inverted depending on coordinate systems (usually V is bottom-up, image is top-down)
        let x = (u * (self.width as f32 - 1.0)) as u32;
        let y = ((1.0 - v) * (self.height as f32 - 1.0)) as u32;

        let pixel = self.image.get_pixel(x, y);
        Color::new(pixel[0], pixel[1], pixel[2])
    }
}
