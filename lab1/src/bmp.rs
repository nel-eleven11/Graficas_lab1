use std::fs::File;
use std::io::{Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32;

pub fn write_bmp_file(
    file_path: &str,  // Ruta del archivo BMP de salida
    buffer: &[u32],  // Datos de píxeles del framebuffer
    width: usize,    // Ancho de la imagen
    height: usize    // Altura de la imagen
) -> std::io::Result<()> {
    // Crear un escritor en búfer para el archivo
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    // Escribir el encabezado BMP
    write_bmp_header(&mut writer, width, height)?;

    // Escribir los datos de píxeles del framebuffer
    write_pixel_data(&mut writer, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    file: &mut BufWriter<File>,  // Escritor en búfer para el archivo
    width: usize,  // Ancho de la imagen
    height: usize  // Altura de la imagen
) -> std::io::Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * (BMP_BITS_PER_PIXEL / 8));
    let pixel_data_size = width * height * (BMP_BITS_PER_PIXEL / 8);
    
    // Escribir la firma BMP
    file.write_all(b"BM")?;
    
    // Escribir el tamaño del archivo, bytes reservados y offset de los píxeles
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0u8; 4])?;  // Bytes reservados
    file.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?;
    
    // Escribir tamaño del encabezado, ancho de la imagen y altura de la imagen
    file.write_all(&[40u8; 4])?;  // Tamaño del encabezado
    file.write_all(&(width as u32).to_le_bytes())?;
    file.write_all(&(height as u32).to_le_bytes())?;
    
    // Escribir planes de color y bits por píxel
    file.write_all(&[1u8, 0])?;  // Planes de color
    file.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?;
    
    // Escribir método de compresión, tamaño de los datos de los píxeles y resolución
    file.write_all(&[0u8; 4])?;  // Sin compresión
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    file.write_all(&[0u8; 16])?;  // Resolución y colores importantes
    
    Ok(())
}

fn write_pixel_data(
    file: &mut BufWriter<File>,  // Escritor en búfer para el archivo
    buffer: &[u32],  // Datos de píxeles del framebuffer
    width: usize,    // Ancho de la imagen
    height: usize    // Altura de la imagen
) -> std::io::Result<()> {
    let padding_size = (4 - (width * 3 % 4)) % 4;
    let padding = vec![0u8; padding_size];
    
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let blue = (pixel & 0xFF) as u8;
            let green = ((pixel >> 8) & 0xFF) as u8;
            let red = ((pixel >> 16) & 0xFF) as u8;
            file.write_all(&[blue, green, red])?;
        }
        file.write_all(&padding)?;
    }
    
    Ok(())
}
