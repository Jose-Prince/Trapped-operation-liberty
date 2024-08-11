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
mod enemy;

use enemy::Enemy;
use framebuffer::Framebuffer;
use texture::Texture;
use color::Color;
use player::Player;
use polygon::Polygon;
use line::Line;
use maze::{render, render3d, render_enemies_pos, render_enemy, draw_player_position, draw_enemies_position, draw_enemy_fov, minimap};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
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
    let (mut maze, player_pos) = render(&mut framebuffer, file_path, 0.5);
    let mut key_down = '\0';

    
    let enemies_pos = render_enemies_pos(&mut framebuffer, file_path);
    let block_size = std::cmp::min(
        framebuffer.get_width() / maze[0].len(),
        framebuffer.get_height() / maze.len()
    ) as f32;
    
    let mut show_fps: bool = false;
    let mut f_key_pressed: bool = false;
    
    let mut enemies: Vec<Enemy> = Vec::new();
    
    for pos in &enemies_pos {
        enemies.push(Enemy::new(*pos, PI/2.0, -10.0, 20.0, (framebuffer.get_height()) as f32));
    }
    
    let mut player = Player::new(player_pos.x, player_pos.y, 0.0, PI / 3.0);
    
    let mut og_pos = player.get_pos();
    let mut new_pos = player.get_pos();

    let texture = Texture::from_file("textures/prison_wall.png");
    let enemy_texture = Texture::from_file("textures/Police.png");

    let mut frame_count = 0;
    let start_time = Instant::now();

    // Inicializa el z_buffer
    let mut z_buffer = vec![f32::INFINITY; framebuffer.get_width()];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            player.update_mouse(mouse_x as f32, mouse_y as f32, width as f32, height as f32);
        }

        (key_down, new_pos) = player.process_events(&window, &maze, block_size, &mut framebuffer);

        framebuffer.clear();

        let mut wall_heights = vec![0; framebuffer.get_width()];

        
        // Renderiza el mapa en 3D
        render3d(&mut framebuffer, &player, &maze, block_size, &texture, &mut wall_heights);

        // Renderiza los enemigos
        for enemy in &enemies {
            render_enemy(
                &mut framebuffer,
                &player,
                &enemy.get_pos(),
                &mut z_buffer,
                &enemy_texture,
                &wall_heights,
                300.0,
                &maze,
                block_size
            );
        }
        maze = minimap(&mut framebuffer, maze.clone(), 0.5, key_down, player.get_a(), og_pos, new_pos);
        
        let delta_time = 1.0 / 30.0;
        
        // Actualiza todos los enemigos
        for enemy in &mut enemies {            
            enemy.update(delta_time, &maze, block_size);
            draw_enemies_position(&mut framebuffer, &enemy.get_pos(), block_size as usize);
            draw_enemy_fov(&mut framebuffer, &enemy, 30, &maze, block_size);
        }

        let (maze, player_pos) = render(&mut framebuffer, file_path, 0.5);

        // Dibuja la posición del jugador en el minimapa
        draw_player_position(&mut framebuffer, player.get_pos(), block_size as usize);

        // Dibuja las posiciones de los enemigos en el minimapa
        
        frame_count += 1;
        let fps = calculate_fps(start_time, frame_count);
        
        if window.is_key_down(Key::F) {
            framebuffer.draw_text(width - 100, 10, &format!("FPS: {:.2}", fps), Color::new(0, 255, 0));
        } else {
            framebuffer.draw_text(width - 100, 10, "", Color::new(0, 255, 0));
        }

        og_pos = new_pos;

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}