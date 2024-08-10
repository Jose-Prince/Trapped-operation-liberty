use crate::polygon::Polygon;
use crate::line::Line;

use crate::enemy::Enemy;
use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::cast_ray::{cast_ray,cast_ray_enemy, cast_ray_2dplayer};
use crate::player::Player;
use crate::texture::Texture;
use crate::Intersect;

use nalgebra_glm::Vec2;
use std::f32::consts::PI;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char, opacity: f32) {
    let color = match cell {
        '+' => Color::new(5, 166, 114),   // Paredes
        '|' | '-' => Color::new(5, 166, 114), // Paredes
        'g' => Color::new(255, 255, 0), // Meta
        ' ' => Color::new(0, 0, 0), // Espacios vacíos
        'p' | 'e' => Color::new(0, 0, 0), // Espacio del jugador
        _ => Color::new(0, 0, 0),        // Color por defecto para caracteres desconocidos
    };

    for y in 0..block_size {
        for x in 0..block_size {
            let bg_color = framebuffer.get_pixel_color((x0 + x) as isize, (y0 + y) as isize);
            let blended_color = color.blend(bg_color.expect("REASON"), opacity);
            if (cell == '|' || cell == '-' || cell == '+') {
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
    wall_heights: &mut Vec<usize>
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
            let stake_height = (block_size as f32 * distance_to_projection_plane / corrected_distance).min(hh * 2.0);

            let stake_top = (hh - (stake_height / 2.0)) as usize;
            let stake_bottom = (hh + (stake_height / 2.0)) as usize;

            wall_heights[i] = stake_bottom;

            let texture_x = ((intersect.x % block_size as f32) / block_size as f32 * texture.width as f32) as usize;

            for y in stake_top..stake_bottom {
                let texture_y = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * texture.height as f32) as usize;
                let color = texture.get_color(texture_x, texture_y);
                framebuffer.set_current_color(color);
                framebuffer.point(i as isize, y as isize);
            }
        }
    }
}

pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y < maze.len() && x < maze[0].len() {
        maze[y][x] == '+' || maze[y][x] == '|' || maze[y][x] == '-'
    } else {
        false
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

pub fn draw_2dplayer_fov(framebuffer: &mut Framebuffer, player: &mut Player, num_rays : usize,  maze: &Vec<Vec<char>>, block_size: f32) {
    for i in 0..num_rays{
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.get_a() - ((PI / 8.0) / 2.0) + ((PI / 8.0) * current_ray);
        cast_ray_enemy(&player.get_pos(), angle, &maze, block_size, true, 100.0, 0.35,Some(framebuffer));
    }
}

pub fn draw_enemy_fov(framebuffer: &mut Framebuffer, enemy: &Enemy, num_rays: usize, maze: &Vec<Vec<char>>, block_size: f32) -> Vec<Option<Intersect>> {
    let mut intersections = Vec::with_capacity(num_rays);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = enemy.get_a() - ((PI / 8.0) / 2.0) + ((PI / 8.0) * current_ray);
        let intersection = cast_ray_enemy(&enemy.get_pos(), -angle, &maze, block_size, true, 100.0, 0.35, Some(framebuffer));
        intersections.push(intersection);
    }

    intersections
}

pub fn minimap(framebuffer: &mut Framebuffer, file_path: &str, opacity: f32, player: &mut Player) -> Vec<Vec<char>> {
    let maze = load_maze(file_path);
    let rows = maze.len();
    let cols = maze[0].len();

    // Escala para hacer el minimapa más pequeño (ajusta el factor según sea necesario)
    let scale_factor = 0.35;
    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);
    let scaled_block_size = (block_size as f32 * scale_factor) as usize;

    let mut player_pos = Vec2::new(0.0, 0.0);

    // Encontrar la posición del jugador
    for row in 0..rows {
        for col in 0..cols {
            let x0 = col * scaled_block_size;
            let y0 = row * scaled_block_size;

            if maze[row][col] == 'p' {
                player_pos = Vec2::new(x0 as f32 + (scaled_block_size / 2) as f32, y0 as f32 + (scaled_block_size / 2) as f32);
            }
        }
    }

    // Definir el número de rayos que se dispararán en el minimapa
    let num_rays = 360;
    let fov = 2.0 * std::f32::consts::PI; // 360 grados

    // Renderizar solo las celdas interceptadas por los rayos
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // Ray proportion
        let direction = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        if let Some(intersect) = cast_ray_2dplayer(
            &player_pos,
            direction,
            &maze,
            scaled_block_size as f32,
            false,
            50.0 as f32,
            1.0,
            None,
        ) {
            println!("{}", player.get_pos().x);
            let col = (intersect.x / scaled_block_size as f32).floor() as usize;
            let row = (intersect.y / scaled_block_size as f32).floor() as usize;
            let x0 = col * scaled_block_size;
            let y0 = row * scaled_block_size;

            // Asegúrate de que la posición está dentro de los límites del laberinto
            if row < rows && col < cols {
                draw_cell(framebuffer, x0, y0, scaled_block_size, maze[row][col], opacity);
            }
        }
    }

    maze
}
