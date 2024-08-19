use crate::polygon::Polygon;
use crate::line::Line;

use crate::enemy::Enemy;
use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::cast_ray::{cast_ray,cast_ray_enemy};
use crate::player::Player;
use crate::texture::Texture;

use std::collections::HashSet;
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration};
use std::thread;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char, opacity: f32) {
    let color = match cell {
        '+' => Color::new(5, 166, 114),   // Paredes
        '|' | '-' | '!' | '/' => Color::new(5, 166, 114), // Paredes
        'g' => Color::new(255, 255, 0), // Meta
        ' ' => Color::new(0, 0, 0), // Espacios vacíos
        'p' | 'e' => Color::new(255, 0, 0), // Espacio del jugador
        _ => Color::new(0, 0, 0),        // Color por defecto para caracteres desconocidos
    };

    for y in 0..block_size {
        for x in 0..block_size {
            let bg_color = framebuffer.get_pixel_color((x0 + x) as isize, (y0 + y) as isize);
            let blended_color = color.blend(bg_color.expect("REASON"), opacity);
            if (cell == '|' || cell == '-' || cell == '+' || cell == '!' || cell == '/') {
                framebuffer.set_current_color(Color::new(5, 166, 114));
            } else {
                framebuffer.set_current_color(blended_color);
            }
            framebuffer.point((x0 + x) as isize, (y0 + y) as isize);
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, file_path: &str, opacity: f32) -> (Vec<Vec<char>>, Vec2) {
    let maze = load_maze(file_path);
    let rows = maze.len();
    let cols = maze[0].len();

    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    let mut player_pos = Vec2::new(0.0, 0.0);

    for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == 'p' {
                player_pos = Vec2::new((col * block_size) as f32 + (block_size / 2) as f32, (row * block_size) as f32 + (block_size / 2) as f32);
            } 
        }
    }

    (maze, player_pos)
}


pub fn render_enemies_pos(framebuffer: &mut Framebuffer, file_path: &str) -> Vec<Vec2> {
    let maze = load_maze(file_path);
    let rows = maze.len();
    let cols = maze[0].len();

    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    let mut enemies_pos: Vec<Vec2> = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == 'e' {
                enemies_pos.push(Vec2::new(
                    (col * block_size) as f32 + (block_size / 2) as f32,
                    (row * block_size) as f32 + (block_size / 2) as f32,
                ));
            }
        }
    }

    enemies_pos
}

pub fn render_enemy(
    framebuffer: &mut Framebuffer,
    player: &Player,
    pos: &Vec2,
    z_buffer: &mut [f32],
    enemy_texture: &Texture,
    wall_heights: &[usize],
    max_sprite_height: f32, // Altura máxima del sprite en la pantalla
    maze: &Vec<Vec<char>>,
    block_size: f32
) {
    let player_a = player.a;

    // Calcular el ángulo del sprite en relación con la dirección del jugador
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);

    // Normalizar el ángulo del sprite
    let normalized_sprite_a = (sprite_a - player_a).atan2(1.0);

    if normalized_sprite_a < -player.fov / 2.0 || normalized_sprite_a > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();

    if sprite_d < 1.0 {
        return;
    }

    // Chequear si hay paredes entre el jugador y el enemigo
    if let Some(intersect) = cast_ray(&player.pos, sprite_a, maze, block_size, false, 1000.0, None) {
        if intersect.distance < sprite_d {
            return; // Hay una pared bloqueando al enemigo
        }
    }

    let screen_height = framebuffer.get_height() as f32;
    let screen_width = framebuffer.get_width() as f32;

    // Calcular el tamaño del sprite en la pantalla
    let sprite_size = ((screen_height / sprite_d) * 40.0).min(max_sprite_height);
    let start_x = (screen_width / 2.0) + (sprite_a - player_a) * (screen_height / player.fov) - (sprite_size / 2.0);

    // Desplazamiento hacia abajo
    let min_offset_down = 25.0;
    let max_offset_down = 200.0; // Ajusta el valor máximo según sea necesario
    let max_distance = 1.0; // Ajusta la distancia máxima según sea necesario

    let offset_down = if sprite_d <= max_distance {
        min_offset_down + (max_offset_down - min_offset_down) * (1.0 - sprite_d / max_distance)
    } else {
        min_offset_down
    };

    // Calcular la posición vertical correcta basándose en las alturas de las paredes
    let ray_index = ((start_x + sprite_size / 2.0) as usize).min(wall_heights.len() - 1);
    let floor_y = wall_heights[ray_index] as f32;
    let start_y = floor_y - sprite_size + offset_down;

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.get_width());
    let end_y = (floor_y as usize).min(framebuffer.get_height());
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if end_x <= 0 || start_x >= framebuffer.get_width() || end_y <= 0 || start_y >= framebuffer.get_height() {
        return;
    }

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = (((x - start_x) * enemy_texture.width as usize) / sprite_size as usize) as u32;
            let ty = (((y - start_y) * enemy_texture.height as usize) / sprite_size as usize) as u32;
            let color = enemy_texture.get_color(tx as usize, ty as usize);

            if color.to_hex() != 0x000000 { // Ajusta el color hexadecimal según sea necesario para el fondo transparente
                framebuffer.set_current_color(color);
                framebuffer.point(x as isize, y as isize);
            }
        }
    }
}


pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: f32,
    texture: &Texture,
    texture_cell: &Texture,
    texture_door: &Texture,
    wall_heights: &mut Vec<usize>,
) {
    let roof_color = Color::new(102, 102, 102);
    let floor_color = Color::new(187, 187, 187);

    let first_half: Vec<[isize; 2]> = vec![
        [0, 0],
        [framebuffer.get_width().try_into().unwrap(), 0],
        [framebuffer.get_width().try_into().unwrap(), (framebuffer.get_height() / 2).try_into().unwrap()],
        [0, (framebuffer.get_height() / 2).try_into().unwrap()],
    ];

    let second_half: Vec<[isize; 2]> = vec![
        [0, (framebuffer.get_height() / 2).try_into().unwrap()],
        [framebuffer.get_width().try_into().unwrap(), (framebuffer.get_height() / 2).try_into().unwrap()],
        [framebuffer.get_width().try_into().unwrap(), framebuffer.get_height().try_into().unwrap()],
        [0, framebuffer.get_height().try_into().unwrap()],
    ];

    let num_rays = framebuffer.get_width();
    let hw = framebuffer.get_width() as f32 / 2.0; // Half width
    let hh = framebuffer.get_height() as f32 / 2.0; // Half height
    let distance_to_projection_plane = hw / (player.fov / 2.0).tan(); // Distancia del jugador al plano de proyección

    framebuffer.polygon(&first_half, roof_color, roof_color);
    framebuffer.polygon(&second_half, floor_color, floor_color);

    wall_heights.clear();
    wall_heights.resize(num_rays, framebuffer.get_height());

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // Ray proportion
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        if let Some(intersect) = cast_ray(&player.pos, angle, maze, block_size, false, 1000.0, None) {
            let distance_to_wall = intersect.distance; // Distance to wall
            let corrected_distance = distance_to_wall * (angle - player.a).cos(); // Correct fish-eye effect
            let stake_height = (block_size * distance_to_projection_plane / corrected_distance).min(hh * 2.0);

            let stake_top = (hh - (stake_height / 2.0)) as usize;
            let stake_bottom = (hh + (stake_height / 2.0)) as usize;

            wall_heights[i] = stake_bottom;

            // Seleccionar la textura basada en el carácter
            let (texture, texture_width, texture_height) = match intersect.character {
                ' ' => (texture, texture.width, texture.height),
                '!' => (texture_cell, texture_cell.width, texture_cell.height),
                '/' => (texture_door, texture_door.width, texture_door.height),
                _ => (texture, texture.width, texture.height),
            };

            let texture_width = texture_width as f32;
            let texture_height = texture_height as f32;

            // Mapeo de textura para la pared
            let texture_x_step = texture_width / block_size;
            let texture_y_step = texture_height / stake_height;

            let wall_x = intersect.x % block_size;
            let mut texture_x = (wall_x * texture_x_step) as usize;

            for y in stake_top..stake_bottom {
                let texture_y = ((y as f32 - stake_top as f32) * texture_y_step) as usize;
                let color = texture.get_color(texture_x, texture_y);
                framebuffer.set_current_color(color);
                framebuffer.point(i as isize, y as isize);
            }
        }
    }
}





pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize) -> (bool, char) {
    if y < maze.len() && x < maze[0].len() {
        return (maze[y][x] == '+' || maze[y][x] == '|' || maze[y][x] == '-' || maze[y][x] == '!' || maze[y][x] == '/', maze[y][x])
    } else {
        return (false, '\0')
    }
}

// Función para dibujar al jugador en el minimapa
pub fn draw_player_position(framebuffer: &mut Framebuffer, player_pos: Vec2, block_size: usize) {
    let player_size = 2;
    let color = Color::new(0, 255, 0); // Verde para el jugador

    for y in -(player_size as isize)..=(player_size as isize) {
        for x in -(player_size as isize)..=(player_size as isize) {
            framebuffer.set_current_color(color);
            framebuffer.point(((player_pos.x * 0.35) as isize + x), ((player_pos.y * 0.35) as isize + y));
        }
    }
}

// Función para dibujar la posición de los enemigos en el minimapa
pub fn draw_enemies_position(framebuffer: &mut Framebuffer, enemies_pos: &Vec2, block_size: usize) {
    let enemy_size = 2;
    let color = Color::new(0, 0, 255); // Rojo para los enemigos

    for y in -(enemy_size as isize)..=(enemy_size as isize) {
        for x in -(enemy_size as isize)..=(enemy_size as isize) {
            framebuffer.set_current_color(color);
            framebuffer.point(((enemies_pos.x * 0.35) as isize + x), ((enemies_pos.y * 0.35) as isize + y));
        }
    }
    
}

pub fn draw_enemy_fov(framebuffer: &mut Framebuffer, enemy: &Enemy, num_rays : usize,  maze: &Vec<Vec<char>>, block_size: f32) {
    for i in 0..num_rays{
        let current_ray = i as f32 / num_rays as f32;
        let angle = enemy.get_a() - ((PI / 8.0) / 2.0) + ((PI / 8.0) * current_ray);
        cast_ray_enemy(&enemy.get_pos(), -angle, &maze, block_size, true, 100.0, 0.35,Some(framebuffer));
    }
}

pub fn minimap(
    framebuffer: &mut Framebuffer,
    mut maze: Vec<Vec<char>>,
    opacity: f32,
    key_down: String,
    direction: f32,
    og_pos: Vec2,
    new_pos: Vec2,
) -> Vec<Vec<char>> {
    let rows = maze.len();
    let cols = maze[0].len();

    let scale_factor = 0.35;
    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);
    let scaled_block_size = (block_size as f32 * scale_factor) as usize;

    // Calcula el tamaño total del minimapa
    let minimap_width = cols * scaled_block_size;
    let minimap_height = rows * scaled_block_size;

    // Dibuja el fondo negro con opacidad
    draw_background(framebuffer, 0, 0, minimap_width, minimap_height, opacity);

    let og_pos_block_x = og_pos.x / block_size as f32;
    let og_pos_block_y = og_pos.y / block_size as f32;
    let new_pos_block_x = new_pos.x / block_size as f32;
    let new_pos_block_y = new_pos.y / block_size as f32;

    if new_pos_block_x.floor() != og_pos_block_x.floor() || new_pos_block_y.floor() != og_pos_block_y.floor() {
        let key_down_str = key_down.as_str();

        if key_down_str == "w" || key_down_str == "s" || key_down_str == "a" || key_down_str == "d" || key_down_str == "wawa" || key_down_str == "wdwd" || key_down_str == "sasa" || key_down_str == "sdsd"{
            maze = update_minimap(maze, key_down_str.to_string(), direction);
        }
    }

    let mut player_pos = Vec2::new(0.0, 0.0);
    let mut player_row = 0;
    let mut player_col = 0;

    for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == 'p' {
                player_row = row;
                player_col = col;
                player_pos = Vec2::new(col as f32 * scaled_block_size as f32, row as f32 * scaled_block_size as f32);
            }
        }
    }

    let visible_radius = 2; // Radio de visión reducido

    for row in (player_row.saturating_sub(visible_radius))..(std::cmp::min(player_row + visible_radius + 1, rows)) {
        for col in (player_col.saturating_sub(visible_radius))..(std::cmp::min(player_col + visible_radius + 1, cols)) {
            let x0 = col * scaled_block_size;
            let y0 = row * scaled_block_size;

            // Dibuja la celda si está en el radio visible o si está en un borde visible según la lógica de visibilidad
            let is_visible = (row as i32 - player_row as i32).abs() <= visible_radius.try_into().unwrap()
                && (col as i32 - player_col as i32).abs() <= visible_radius.try_into().unwrap();

            let is_border_cell = row == 0 || row == rows - 1 || col == 0 || col == cols - 1;

            if is_visible || is_border_cell {
                draw_cell(framebuffer, x0, y0, scaled_block_size, maze[row][col], opacity);
            }
        }
    }

    maze
}


// Función para dibujar el fondo negro con opacidad
fn draw_background(framebuffer: &mut Framebuffer, x: usize, y: usize, width: usize, height: usize, opacity: f32) {
    let color = Color::new(0,0,0);
    for i in 0..width {
        for j in 0..height {
            let bg_color = framebuffer.get_pixel_color((x + i) as isize, (y + j) as isize);
            let blended_color = color.blend(bg_color.expect("REASON"), 0.7);
            
            framebuffer.set_current_color(blended_color);

            framebuffer.point((x + i).try_into().unwrap(), (y + j).try_into().unwrap());
        }
    }
}



fn update_minimap(mut maze: Vec<Vec<char>>, key_down: String, direction: f32) -> Vec<Vec<char>> {
    let mut x_dir = direction.cos().round() as isize;  
    let mut y_dir = direction.sin().round() as isize;

    // Ajusta la dirección basada en la tecla presionada
    match key_down.as_str() {
        "w" => { /* No se cambia la dirección */ },
        "s" => {
            x_dir = -x_dir;
            y_dir = -y_dir;
        },
        "a" => {
            let angle = direction - std::f32::consts::FRAC_PI_2; // Gira 90 grados hacia la izquierda
            x_dir = angle.cos().round() as isize;
            y_dir = angle.sin().round() as isize;
        },
        "d" => {
            let angle = direction + std::f32::consts::FRAC_PI_2; // Gira 90 grados hacia la derecha
            x_dir = angle.cos().round() as isize;
            y_dir = angle.sin().round() as isize;
        },
        "wawa" => {
            let angle = direction - std::f32::consts::FRAC_PI_4; // Movimiento hacia adelante y diagonal
            x_dir = (angle.cos() * 1.0).round() as isize;
            y_dir = (angle.sin() * 1.0).round() as isize;

        },
        "wdwd" => {
            let angle = direction + std::f32::consts::FRAC_PI_4; // Movimiento diagonal hacia adelante y hacia la derecha
            x_dir = (angle.cos() * 1.0).round() as isize;
            y_dir = (angle.sin() * 1.0).round() as isize;
        },
        "sasa" => {
            let angle = direction - std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_4; // Movimiento diagonal hacia adelante y hacia la izquierda
            x_dir = (angle.cos() * 1.0).round() as isize;
            y_dir = (angle.sin() * 1.0).round() as isize;
        },
        "sdsd" => {
            let angle = direction + std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_4; // Movimiento diagonal hacia atrás y hacia la derecha
            x_dir = (angle.cos() * 1.0).round() as isize;
            y_dir = (angle.sin() * 1.0).round() as isize;
        },
        _ => { return maze; } // Si no es una tecla válida, no hacer nada
    }

    // Encuentra la posición de 'p'
    let mut p_pos = None;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            if maze[row][col] == 'p' {
                p_pos = Some((row as isize, col as isize));
                break;
            }
        }
    }

    if let Some((row, col)) = p_pos {
        let new_row = row + y_dir;
        let new_col = col + x_dir;

        // Verifica que la nueva posición esté dentro de los límites y sea un espacio vacío
        if new_row >= 0 && new_row < maze.len() as isize &&
           new_col >= 0 && new_col < maze[0].len() as isize &&
           (maze[new_row as usize][new_col as usize] == ' ' || maze[new_row as usize][new_col as usize] == 'e'){
            maze[new_row as usize][new_col as usize] = 'p';
            maze[row as usize][col as usize] = ' ';
        }
    }

    maze
}
