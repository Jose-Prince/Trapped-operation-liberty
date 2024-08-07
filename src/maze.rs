use crate::polygon::Polygon;
use crate::line::Line;

use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::cast_ray::cast_ray;
use crate::player::Player;
use crate::texture::Texture;

use nalgebra_glm::Vec2;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    match cell {
        '+' => framebuffer.set_current_color(Color::new(255, 0, 0)), // Paredes
        '|' | '-' => framebuffer.set_current_color(Color::new(255, 0, 0)), // Paredes
        'g' => framebuffer.set_current_color(Color::new(255, 255, 0)), // Meta
        ' ' => framebuffer.set_current_color(Color::new(255, 255, 255)), // Espacios vacíos
        _ => framebuffer.set_current_color(Color::new(0, 0, 0)), // Color por defecto para caracteres desconocidos
    }

    for y in 0..block_size {
        for x in 0..block_size {
            framebuffer.point((x0 + x) as isize, (y0 + y) as isize);
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, file_path: &str) -> (Vec<Vec<char>>, Vec2) {
    let maze = load_maze(file_path);
    let rows = maze.len();
    let cols = maze[0].len();

    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    let mut player_pos = Vec2::new(0.0, 0.0);

    for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == 'p' {
                player_pos = Vec2::new((col * block_size) as f32 + (block_size / 2) as f32, (row * block_size) as f32 + (block_size / 2) as f32);
            } else {
                draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
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
    enemy_texture: &Texture
) {
    let player_a = player.a;

    // Calcular el ángulo del sprite en relación con la dirección del jugador
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);

    if sprite_a < player_a - (player.fov / 2.0) || sprite_a > player_a + (player.fov / 2.0) {
        return;
    }

    let sprite_d = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();
    
    if sprite_d < 1.0 {
        return;
    }

    let screen_height = framebuffer.get_height() as f32;
    let screen_width = framebuffer.get_width() as f32;

    // Calcular el tamaño del sprite en la pantalla
    let sprite_size = (screen_height / sprite_d) * 100.0;
    let start_x = (screen_width / 2.0) + (sprite_a - player_a) * (screen_height / player.fov) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.get_width());
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.get_height());
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

            if color.to_hex() != 0x980088 { // Ajusta el color hexadecimal según sea necesario
                framebuffer.set_current_color(color);
                framebuffer.point(x as isize, y as isize);
            }
            
        }
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: f32, texture: &Texture) {
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

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // Ray proportion
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        if let Some(intersect) = cast_ray(&player.pos, angle, maze, block_size, false, None) {
            let distance_to_wall = intersect.distance; // Distance to wall
            let corrected_distance = distance_to_wall * (angle - player.a).cos(); // Correct fish-eye effect
            let stake_height = (block_size as f32 * distance_to_projection_plane / corrected_distance).min(hh * 2.0);

            let stake_top = (hh - (stake_height / 2.0)) as usize;
            let stake_bottom = (hh + (stake_height / 2.0)) as usize;

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
