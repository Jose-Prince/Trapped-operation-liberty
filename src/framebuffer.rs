use crate::bmp::write_bmp_file;
use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0);
        let current_color = Color::new(255, 255, 255);

        let buffer = vec![background_color.to_hex(); width * height];

        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        let color_hex = self.background_color.to_hex();
        self.buffer.fill(color_hex);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (self.width * y as usize) + x as usize;
            self.buffer[index] = self.current_color.to_hex();
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_point(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (self.width * y as usize) + x as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn save_as_bmp(&self, file_path: &str) -> std::io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }

    pub fn fill_polygon(&mut self, vertices: &Vec<[isize; 2]>, fill_color: Color) {
        let min_y = vertices.iter().map(|v| v[1]).min().unwrap_or(0);
        let max_y = vertices.iter().map(|v| v[1]).max().unwrap_or(0);

        for y in min_y..=max_y {
            let mut nodes = vec![];
            let mut j = vertices.len() - 1;
            for i in 0..vertices.len() {
                let vi = vertices[i];
                let vj = vertices[j];
                if vi[1] < y && vj[1] >= y || vj[1] < y && vi[1] >= y {
                    let intersect_x = vi[0] + (y - vi[1]) * (vj[0] - vi[0]) / (vj[1] - vi[1]);
                    nodes.push(intersect_x);
                }
                j = i;
            }
            nodes.sort();
            for n in (0..nodes.len()).step_by(2) {
                if n + 1 < nodes.len() {
                    for x in nodes[n]..=nodes[n + 1] {
                        self.set_current_color(fill_color);
                        self.point(x, y);
                    }
                }
            }
        }
    }
}
