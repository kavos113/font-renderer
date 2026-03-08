use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<bool>,
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
