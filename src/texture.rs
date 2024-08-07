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
            let rgba = pixel.2 .0;
            data.push(Color::new(rgba[0] as i32, rgba[1] as i32, rgba[2] as i32));
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
