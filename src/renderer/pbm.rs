use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<bool>,
}

pub struct ImageWithSpace {
    pub image: Image,
    pub space: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            data: vec![false; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: bool) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.data[index] = value;
        }
    }

    // concat. align base line
    pub fn concat(&self, other: &Image, space: u32) -> Image {
        let new_width = self.width + other.width + space;
        let new_height = self.height.max(other.height);
        let mut new_data = vec![false; (new_width * new_height) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                new_data[(y * new_width + x) as usize] = self.data[index];
            }
        }

        for y in 0..other.height {
            for x in 0..other.width {
                let index = (y * other.width + x) as usize;
                new_data[(y * new_width + self.width + space + x) as usize] = other.data[index];
            }
        }

        Image {
            width: new_width,
            height: new_height,
            data: new_data,
        }
    }

    pub fn concat_all(images: &[ImageWithSpace]) -> Image {
        let total_width: u32 = images
            .iter()
            .map(|i| i.image.width + i.space)
            .sum();
        let max_height: u32 = images
            .iter()
            .map(|i| i.image.height)
            .max()
            .unwrap_or(0);

        let mut new_data = vec![false; (total_width * max_height) as usize];
        let mut current_x = 0;

        for image_with_space in images {
            for y in 0..image_with_space.image.height {
                for x in 0..image_with_space.image.width {
                    let index = (y * image_with_space.image.width + x) as usize;
                    new_data[(y * total_width + current_x + x) as usize] = image_with_space.image.data[index];
                }
            }
            current_x += image_with_space.image.width + image_with_space.space;
        }

        Image {
            width: total_width,
            height: max_height,
            data: new_data,
        }
    }

    pub fn write_to_pbm(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        // Write PBM header
        writer.write_all(format!("P1\n{} {}\n", self.width, self.height).as_bytes())?;

        // Write pixel data
        for y in 0..self.height {
            for x in 0..self.width {
                let value = if self.data[(y * self.width + x) as usize] { '1' } else { '0' };
                writer.write_all(value.to_string().as_bytes())?;
            }
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    pub fn write_to_ppm(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        // Write PPM header
        writer.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        // Write pixel data
        for y in 0..self.height {
            for x in 0..self.width {
                let value = if self.data[(y * self.width + x) as usize] { "255 255 255" } else { "0 0 0" };
                writer.write_all(value.as_bytes())?;
                writer.write_all(b" ")?;
            }
            writer.write_all(b"\n")?;
        }

        Ok(())
    }
}
