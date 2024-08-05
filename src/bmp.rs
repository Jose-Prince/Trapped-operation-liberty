use std::fs::File;
use std::io::{Write, BufWriter, Result};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32;

pub fn write_bmp_file(
    file_path: &str,
    buffer: &[u32],
    width: usize,
    height: usize,
) -> Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;

    writer.flush()?;
    Ok(())
}

fn write_bmp_header(
    writer: &mut BufWriter<File>,
    width: usize,
    height: usize,
) -> Result<()> {
    let file_size = (BMP_HEADER_SIZE + (width * height * 4)) as u32;
    let reserved: u32 = 0;
    let offset: u32 = BMP_PIXEL_OFFSET as u32;

    // BMP header
    writer.write_all(b"BM")?;
    writer.write_all(&file_size.to_le_bytes())?;
    writer.write_all(&reserved.to_le_bytes())?;
    writer.write_all(&offset.to_le_bytes())?;

    // DIB header
    let header_size: u32 = 40;
    let planes: u16 = 1;
    let bpp: u16 = BMP_BITS_PER_PIXEL as u16;
    let compression: u32 = 0;
    let image_size: u32 = (width * height * 4) as u32;
    let ppm: u32 = 2835; // 72 DPI

    writer.write_all(&header_size.to_le_bytes())?;
    writer.write_all(&(width as u32).to_le_bytes())?;
    writer.write_all(&(height as u32).to_le_bytes())?;
    writer.write_all(&planes.to_le_bytes())?;
    writer.write_all(&bpp.to_le_bytes())?;
    writer.write_all(&compression.to_le_bytes())?;
    writer.write_all(&image_size.to_le_bytes())?;
    writer.write_all(&ppm.to_le_bytes())?;
    writer.write_all(&ppm.to_le_bytes())?;
    writer.write_all(&0u32.to_le_bytes())?; // Number of colors
    writer.write_all(&0u32.to_le_bytes())?; // Important colors

    Ok(())
}

fn write_pixel_data(
    writer: &mut BufWriter<File>,
    buffer: &[u32],
    width: usize,
    height: usize,
) -> Result<()> {
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let b = (pixel & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let a = ((pixel >> 24) & 0xFF) as u8;
            writer.write_all(&[b, g, r, a])?;
        }
    }
    Ok(())
}
