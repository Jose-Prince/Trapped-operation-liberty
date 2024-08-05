mod framebuffer;
mod color;
mod fileReader;
mod bmp;
mod maze;
mod player;
mod cast_ray;
mod texture;

use framebuffer::Framebuffer;
use texture::Texture;
use color::Color;
use player::Player;
use maze::render;
use maze::render3d;
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use std::f32::consts::PI;

fn main() {
    // Suponiendo que tienes un framebuffer y un jugador configurados
    let width = 800;
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
    let maze = render(&mut framebuffer, file_path);
    let block_size = std::cmp::min(
        framebuffer.get_width() / maze[0].len(),
        framebuffer.get_height() / maze.len()
    );

    let mut player = Player::new(100.0, 100.0, 0.0, PI / 3.0);

    // Cargar la textura desde un archivo .png
    let texture = Texture::from_file("textures/prison_wall.png");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        player.process_events(&window, &maze, block_size, &mut framebuffer);

        framebuffer.clear();
        player.draw(&mut framebuffer);
        
        render3d(&mut framebuffer, &player, "src/maze.txt", &texture);

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    // Renderizar la vista 3D con la textura
}
