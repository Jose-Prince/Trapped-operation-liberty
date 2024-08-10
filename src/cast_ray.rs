use nalgebra_glm::Vec2;
use crate::color::Color;
use crate::framebuffer::Framebuffer;

pub struct Intersect {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
}

pub fn cast_ray(
    player_pos: &Vec2,
    direction: f32,
    maze: &Vec<Vec<char>>,
    block_size: f32,
    draw_line: bool,
    max_distance: f32,
    mut framebuffer: Option<&mut Framebuffer>,
) -> Option<Intersect> {
    let mut d = 0.0;

    let cos = direction.cos();
    let sin = direction.sin();

    loop {
        let x = (player_pos.x + cos * d) as usize;
        let y = (player_pos.y + sin * d) as usize;

        // Convertir block_size a usize para la división
        let block_size_usize = block_size as usize;

        let i = (x as f32 / block_size).floor() as usize;
        let j = (y as f32 / block_size).floor() as usize;

        // Verificar que los índices están dentro de los límites
        if j >= maze.len() || i >= maze[0].len() {
            return None;
        }

        // Verificar si la celda no es un espacio vacío y no es el punto 'p'
        if maze[j][i] != ' ' && maze[j][i] != 'p' &&maze[j][i] != 'e' {
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

pub fn cast_ray_enemy(
    player_pos: &Vec2,
    direction: f32,
    maze: &Vec<Vec<char>>,
    block_size: f32,
    draw_line: bool,
    max_distance: f32,
    scale: f32,
    mut framebuffer: Option<&mut Framebuffer>,
) -> Option<Intersect> {
    let mut d = 0.0;

    let cos = direction.cos();
    let sin = direction.sin();

    loop {
        let x = (player_pos.x * scale + cos * d) as usize;
        let y = (player_pos.y * scale + sin * d) as usize;

        let i = (x as f32 / block_size).floor() as usize;
        let j = (y as f32 / block_size).floor() as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return None;
        }


            if let Some(fb) = framebuffer.as_deref() {
                if let Some(hex_color) = fb.get_point(x as isize, y as isize) {
                    let r = ((hex_color >> 16) & 0xFF) as i32;
                    let g = ((hex_color >> 8) & 0xFF) as i32;
                    let b = (hex_color & 0xFF) as i32;
                    
                    let color = Color::new(r, g, b);

                    // Verificar si el color es solo rojo
                    if color.match_rgb() {
                        return Some(Intersect {
                            x: x as f32,
                            y: y as f32,
                            distance: d,
                        });
                    }
                }
            }
        

        if draw_line {
            if let Some(fb) = framebuffer.as_deref_mut() {
                fb.point(x as isize, y as isize);
            }
        }

        d += 1.0;

        if d > max_distance * scale {
            return None;
        }
    }
}

pub fn cast_ray_2dplayer(
    player_pos: &Vec2,
    direction: f32,
    maze: &Vec<Vec<char>>,
    block_size: f32,
    draw_line: bool,
    max_distance: f32,
    scale: f32,
    mut framebuffer: Option<&mut Framebuffer>,
) -> Option<Intersect> {
    let mut d = 0.0;

    // Calcular las componentes de la dirección usando el ángulo correcto
    let cos = direction.cos();
    let sin = direction.sin();

    loop {
        let x = player_pos.x + cos * d;
        let y = player_pos.y + sin * d;

        let i = (x / block_size).floor() as usize;
        let j = (y / block_size).floor() as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return None;
        }

        if maze[j][i] != ' ' && maze[j][i] != 'p' {
            return Some(Intersect {
                x,
                y,
                distance: d,
            });
        }

        if draw_line {
            if let Some(fb) = framebuffer.as_deref_mut() {
                fb.point(x as isize, y as isize);
            }
        }

        d += 1.0;

        if d > max_distance * scale {
            return None;
        }
    }
}
