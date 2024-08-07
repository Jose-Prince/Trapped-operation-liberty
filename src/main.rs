mod framebuffer;
mod color;
mod fileReader;
mod bmp;
mod maze;
mod player;
mod cast_ray;
mod texture;
mod polygon;
mod line;

use framebuffer::Framebuffer;
use texture::Texture;
use color::Color;
use player::Player;
use polygon::Polygon;
use line::Line;
use maze::{render, render3d, render_enemies_pos, render_billboard};
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

fn calculate_fps(start_time: Instant, frame_count: usize) -> f64 {
    let duration = start_time.elapsed().as_secs_f64();
    frame_count as f64 / duration
}

fn main() {
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
    let (maze, player_pos) = render(&mut framebuffer, file_path);
    let enemies_pos = render_enemies_pos(& mut framebuffer, file_path);
    let block_size = std::cmp::min(
        framebuffer.get_width() / maze[0].len(),
        framebuffer.get_height() / maze.len()
    );

    let mut player = Player::new(player_pos.x, player_pos.y, 0.0, PI / 3.0);

    // Cargar la textura desde un archivo .png
    let texture = Texture::from_file("textures/prison_wall.png");
    let enemy_texture = Texture::from_file("textures/Police.png");

    let mut frame_count = 0;
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Obtener la posición del mouse
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            player.update_mouse(mouse_x as f32, mouse_y as f32, width as f32, height as f32);
        }

        player.process_events(&window, &maze, block_size, &mut framebuffer);

        framebuffer.clear();
        
        render3d(&mut framebuffer, &player, &maze, block_size, &texture);
        render_billboard(&mut framebuffer, &player.pos, player.a, &enemies_pos[0], &enemy_texture, player.fov);

        frame_count += 1;
        let fps = calculate_fps(start_time, frame_count);
        framebuffer.draw_text(width - 100, 10, &format!("FPS: {:.2}", fps), Color::new(0, 255, 0));

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
