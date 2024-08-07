use image::io::Reader as ImageReader;
use image::GenericImageView;
use crate::color::Color;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
}

impl Texture {
    pub fn new(width: usize, height: usize, data: Vec<Color>) -> Self {
        Texture{ width, height, data }
    }

    pub fn from_file(path: &str) -> Self {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let (width, height) = img.dimensions();
        let mut data = Vec::new();
    
        for pixel in img.pixels() {
            let (r, g, b, a) = (pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2], pixel.2 .0[3]);
            if a > 0 { // Verificar si el píxel no es completamente transparente
                data.push(Color::new(r as i32, g as i32, b as i32));
            } else {
                data.push(Color::new(0, 0, 0)); // Fondo transparente
            }
        }
    
        Texture {
            width: width as usize,
            height: height as usize,
            data,
        }
    }    

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            Color::new(0,0,0)
        }
    }
}
