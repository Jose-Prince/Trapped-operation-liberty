use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::cast_ray::cast_ray; // Asegúrate de importar la función cast_ray correctamente
use crate::player::Player;
use crate::texture::Texture;

use nalgebra_glm::Vec2;
use std::io::Result;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    match cell {
        '+' => framebuffer.set_current_color(Color::new(255, 0, 0)), // Paredes
        '|' | '-' => framebuffer.set_current_color(Color::new(255, 0, 0)), // Paredes
        'p' => framebuffer.set_current_color(Color::new(0, 0, 255)), // Punto de inicio
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

pub fn render(framebuffer: &mut Framebuffer, file_path: &str) -> Vec<Vec<char>> {
    let maze = load_maze(file_path);
    let rows = maze.len();
    let cols = maze[0].len();

    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    let mut player_pos = Vec2::new(0.0, 0.0);

    for row in 0..rows {
        for col in 0..cols {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
            if maze[row][col] == 'p' {
                player_pos = Vec2::new((col * block_size) as f32 + (block_size / 2) as f32, (row * block_size) as f32 + (block_size / 2) as f32);
            }
        }
    }

    maze
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, file_path: &str, texture: &Texture) -> Vec<Vec<char>> {
    let maze = load_maze(file_path);
    let rows = maze.len();
    let cols = maze[0].len();

    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    let num_rays = framebuffer.get_width();
    let hw = framebuffer.get_width() as f32 / 2.0; // Half width
    let hh = framebuffer.get_height() as f32 / 2.0; // Half height
    let distance_to_projection_plane = hw / (player.fov / 2.0).tan(); // Distancia del jugador al plano de proyección

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // Ray proportion
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        if let Some(intersect) = cast_ray(&player.pos, angle, &maze, block_size, false, None) {
            // Calculate the height of the stake
            let distance_to_wall = intersect.distance; // Distance to wall
            let corrected_distance = distance_to_wall * (angle - player.a).cos(); // Correct fish-eye effect
            let stake_height = (block_size as f32 * distance_to_projection_plane / corrected_distance).min(hh * 2.0);
            
            // Calculate stake top and bottom
            let stake_top = (hh - (stake_height / 2.0)) as usize;
            let stake_bottom = (hh + (stake_height / 2.0)) as usize;

            // Calculate the texture column to use
            let texture_x = ((intersect.x % block_size as f32) / block_size as f32 * texture.width as f32) as usize;
            
            // Draw the stake with texture
            for y in stake_top..stake_bottom {
                let texture_y = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * texture.height as f32) as usize;
                let color = texture.get_color(texture_x, texture_y);
                framebuffer.set_current_color(color);
                framebuffer.point(i as isize, y as isize);
            }
        }
    }

    maze
}


pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y < maze.len() && x < maze[0].len() {
        maze[y][x] == '+' || maze[y][x] == '|' || maze[y][x] == '-'
    } else {
        false
    }
}