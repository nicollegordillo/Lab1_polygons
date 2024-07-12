use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::framebuffer::Framebuffer;

pub struct Bitmap {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Bitmap {
    // Create a new Bitmap from a Framebuffer
    pub fn from_framebuffer(framebuffer: &Framebuffer) -> Bitmap {
        let width = framebuffer.width;
        let height = framebuffer.height;
        let pixels = framebuffer.pixels.clone(); // Clone the framebuffer pixels

        Bitmap { width, height, pixels }
    }

    // Save Bitmap to file in BMP format
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let file = File::create(&path)?;

        let mut writer = io::BufWriter::new(file);

        // Write BMP file headers
        self.write_file_header(&mut writer)?;
        self.write_info_header(&mut writer)?;

        // Write pixel data (BGR format)
        self.write_pixels(&mut writer)?;

        writer.flush()?;
        Ok(())
    }

    // Write BMP file header
    fn write_file_header(&self, writer: &mut io::BufWriter<File>) -> io::Result<()> {
        let file_size = 54 + (self.width * self.height * 3); // Total file size in bytes
        let reserved_bytes: [u8; 4] = [0; 4];
        let data_offset = 54; // Offset to pixel data

        let file_header = [
            b'B', b'M',                       // Magic bytes
            file_size as u8, (file_size >> 8) as u8, (file_size >> 16) as u8, (file_size >> 24) as u8, // File size
            reserved_bytes[0], reserved_bytes[1], reserved_bytes[2], reserved_bytes[3], // Reserved bytes
            data_offset as u8, (data_offset >> 8) as u8, (data_offset >> 16) as u8, (data_offset >> 24) as u8, // Data offset
        ];

        writer.write_all(&file_header)?;
        Ok(())
    }

    // Write BMP info header
    fn write_info_header(&self, writer: &mut io::BufWriter<File>) -> io::Result<()> {
        let info_header = [
            40, 0, 0, 0,                       // Info header size
            (self.width as i32) as u8, ((self.width as i32) >> 8) as u8, ((self.width as i32) >> 16) as u8, ((self.width as i32) >> 24) as u8, // Width
            (self.height as i32) as u8, ((self.height as i32) >> 8) as u8, ((self.height as i32) >> 16) as u8, ((self.height as i32) >> 24) as u8, // Height
            1, 0,                              // Number of color planes
            24, 0,                             // Bits per pixel (24-bit RGB)
            0, 0, 0, 0,                        // Compression method (0 for uncompressed)
            0, 0, 0, 0,                        // Image size (0 if uncompressed)
            0, 0, 0, 0,                        // Horizontal resolution (pixels per meter)
            0, 0, 0, 0,                        // Vertical resolution (pixels per meter)
            0, 0, 0, 0,                        // Number of colors in color palette (0 for full color image)
            0, 0, 0, 0,                        // Number of important colors (0 when every color is important)
        ];

        writer.write_all(&info_header)?;
        Ok(())
    }

    // Write BMP pixel data (BGR format)
    fn write_pixels(&self, writer: &mut io::BufWriter<File>) -> io::Result<()> {
        // BMP pixels are stored bottom-to-top and left-to-right
        let mut bmp_data = Vec::new();
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.pixels[y * self.width + x];
                let blue = (pixel & 0xFF) as u8;
                let green = ((pixel >> 8) & 0xFF) as u8;
                let red = ((pixel >> 16) & 0xFF) as u8;
                bmp_data.push(blue);
                bmp_data.push(green);
                bmp_data.push(red);
            }
        }

        writer.write_all(&bmp_data)?;
        Ok(())
    }
}