use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Option<Intersect> {
    let mut d = 0.0;
    let max_distance = 1000.0; // Límite máximo de distancia

    framebuffer.set_current_color(Color::new(255, 0, 0));

    loop {
        let cos = a.cos();
        let sin = a.sin();
        let x = (player.pos.x + cos * d) as usize;
        let y = (player.pos.y + sin * d) as usize;

        let i = x / block_size;
        let j = y / block_size;

        // Verificar que los índices están dentro de los límites
        if j >= maze.len() || i >= maze[0].len() {
            return None;
        }

        if maze[j][i] != ' ' {
            return Some(Intersect {
                distance: d,
                impact: maze[j][i],
            });
        }

        if draw_line {
            framebuffer.point(x as isize, y as isize);
        }

        d += 1.0;

        // Limitar la distancia máxima para evitar bucles infinitos
        if d > max_distance {
            return None;
        }
    }
}
