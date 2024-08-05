use crate::framebuffer::Framebuffer;
use nalgebra_glm as glm; // Asumiendo que nalgebra-glm está correctamente importado y configurado

pub trait Line {
    fn line(&mut self, vertex1: glm::TVec3<f64>, vertex2: glm::TVec3<f64>);
}

impl Line for Framebuffer {
    fn line(&mut self, vertex1: glm::TVec3<f64>, vertex2: glm::TVec3<f64>) {
        let x1 = vertex1.x.round() as isize;
        let y1 = vertex1.y.round() as isize;
        let x2 = vertex2.x.round() as isize;
        let y2 = vertex2.y.round() as isize;

        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x, y);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                if x == x2 {
                    break;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 {
                    break;
                }
                err += dx;
                y += sy;
            }
        }
    }
}
