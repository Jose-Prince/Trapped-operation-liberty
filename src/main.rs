mod framebuffer;
mod color;
mod fileReader;
mod bmp;
mod maze;
mod player;
mod cast_ray;

use framebuffer::Framebuffer;
use color::Color;
use player::Player;
use maze::render;
use maze::render3d;
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use std::f32::consts::PI;

fn main() {
    let width = 1000;
    let height = 800;
    let mut framebuffer = Framebuffer::new(width, height);

    let mut window = Window::new(
        "Maze",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let file_path = "src/maze.txt";
    let maze = render(&mut framebuffer, file_path); // `render` ahora devuelve `Vec<Vec<char>>`
    let block_size = std::cmp::min(
        framebuffer.get_width() / maze[0].len(), 
        framebuffer.get_height() / maze.len()
    );
    let mut player = Player::new(100.0, 100.0, 0.0, PI / 4.0); // Posición inicial del jugador

    while window.is_open() && !window.is_key_down(Key::Escape) {
        player.process_events(&window, &maze, block_size);

        framebuffer.clear();
        player.draw(&mut framebuffer);
        
        // let num_rays = 5;
        // for i in 0..num_rays {
        //     let current_ray = i as f32 / num_rays as f32;
        //     let angle = player.a - ((PI/8.0) / 2.0) + ((PI/8.0) * current_ray);
        //     cast_ray::cast_ray(&mut framebuffer, &maze, &player, angle, block_size);
        // }

        render3d(&mut framebuffer, &player, file_path);

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}