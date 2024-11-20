// bmp.rs

use std::fs::File;
use std::io::{Write, BufWriter};

use crate::framebuffer::Framebuffer;

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(file_path: &str, buffer: &[u32], width: usize, height: usize) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(file_path)?);
    write_bmp_header(&mut file, width, height)?;
    write_pixel_data(&mut file, buffer, width, height)?;
    Ok(())
}

fn write_bmp_header(file: &mut BufWriter<File>, width: usize, height: usize) -> std::io::Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * BMP_BITS_PER_PIXEL / 8) as usize;
    let pixel_data_size = file_size - BMP_HEADER_SIZE;

    file.write_all(b"BM")?; // BMP signature
    file.write_all(&(file_size as u32).to_le_bytes())?; // File size
    file.write_all(&0u32.to_le_bytes())?; // Reserved
    file.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?; // Pixel data offset
    file.write_all(&40u32.to_le_bytes())?; // Header size
    file.write_all(&(width as u32).to_le_bytes())?; // Image width
    file.write_all(&(height as u32).to_le_bytes())?; // Image height
    file.write_all(&1u16.to_le_bytes())?; // Color planes
    file.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?; // Bits per pixel
    file.write_all(&0u32.to_le_bytes())?; // Compression method
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?; // Pixel data size
    file.write_all(&0u32.to_le_bytes())?; // Horizontal resolution
    file.write_all(&0u32.to_le_bytes())?; // Vertical resolution
    file.write_all(&0u32.to_le_bytes())?; // Number of colors
    file.write_all(&0u32.to_le_bytes())?; // Important colors

    Ok(())
}

fn write_pixel_data(file: &mut BufWriter<File>, buffer: &[u32], width: usize, height: usize) -> std::io::Result<()> {
    let padding_size = (4 - (width * BMP_BITS_PER_PIXEL / 8) % 4) % 4;
    let padding = [0u8; 3];

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let bgr = [(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8];
            file.write_all(&bgr)?;
        }
        file.write_all(&padding[..padding_size])?;
    }

    Ok(())
}


pub trait WriteBmp {
    fn render_buffer(&self, file_path: &str) -> std::io::Result<()>;
}

impl WriteBmp for Framebuffer {
    fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}