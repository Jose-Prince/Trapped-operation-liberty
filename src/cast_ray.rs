use nalgebra_glm::Vec2;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
}

pub fn cast_ray(
    player_pos: &Vec2,
    direction: f32,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    draw_line: bool,
    mut framebuffer: Option<&mut Framebuffer>,
) -> Option<Intersect> {
    let mut d = 0.0;
    let max_distance = 1000.0; // Límite máximo de distancia

    let cos = direction.cos();
    let sin = direction.sin();

    loop {
        let x = (player_pos.x + cos * d) as usize;
        let y = (player_pos.y + sin * d) as usize;

        let i = x / block_size;
        let j = y / block_size;

        // Verificar que los índices están dentro de los límites
        if j >= maze.len() || i >= maze[0].len() {
            return None;
        }

        if maze[j][i] != ' ' {
            return Some(Intersect {
                x: x as f32,
                y: y as f32,
                distance: d,
            });
        }

        if draw_line {
            if let Some(fb) = framebuffer.as_deref_mut() {
                fb.point(x as isize, y as isize);
            }
        }

        d += 1.0;

        // Limitar la distancia máxima para evitar bucles infinitos
        if d > max_distance {
            return None;
        }
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    let two_pi = std::f32::consts::PI * 2.0;
    (angle + two_pi) % two_pi
}
