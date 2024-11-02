use crate::Color;
use image::{ImageBuffer, Rgb};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn save_as_png(&self, file_path: &str) -> Result<(), image::ImageError> {
        let mut img = ImageBuffer::new(self.width, self.height);

        for (i, color) in self.pixels.iter().enumerate() {
            let x = (i as u32) % self.width;
            let y = (i as u32) / self.width;
            img.put_pixel(
                x, 
                y, 
                Rgb([
                    (color.vector.x * 255.0) as u8,
                    (color.vector.y * 255.0) as u8,
                    (color.vector.z * 255.0) as u8,
                ])
            );
        }

        img.save(file_path)?;
        Ok(())
    }
} 