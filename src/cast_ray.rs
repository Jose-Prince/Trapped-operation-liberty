use nalgebra_glm::Vec2;
use crate::color::Color;
use crate::framebuffer::Framebuffer;

pub struct Intersect {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub character: char,  // Añadido para almacenar el carácter encontrado
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

    let maze_height = maze.len();
    let maze_width = maze[0].len();

    loop {
        let x = (player_pos.x + cos * d) as f32;
        let y = (player_pos.y + sin * d) as f32;

        // Convertir coordenadas del mundo a índices de la cuadrícula del laberinto
        let i = (x / block_size).floor() as isize;
        let j = (y / block_size).floor() as isize;

        // Verificar que las coordenadas están dentro de los límites del laberinto
        if j < 0 || j >= maze_height as isize || i < 0 || i >= maze_width as isize {
            return None;
        }

        // Obtener el carácter en la celda
        let cell_char = maze[j as usize][i as usize];

        // Verificar si la celda no es un espacio vacío y no es el punto 'p'
        if cell_char != ' ' && cell_char != 'p' && cell_char != 'e' {
            return Some(Intersect {
                x,
                y,
                distance: d,
                character: cell_char,  // Añadir el carácter encontrado
            });
        }

        // Dibujar la línea si se solicita
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
                            character: '\0',
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