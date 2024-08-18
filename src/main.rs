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
mod audioPlayer;
mod scenes;

use enemy::Enemy;
use framebuffer::Framebuffer;
use texture::Texture;
use color::Color;
use player::Player;
use polygon::Polygon;
use audioPlayer::AudioPlayer;
use line::Line;
use scenes::{game_start};
use maze::{render, render3d, render_enemies_pos, render_enemy, draw_player_position, draw_enemies_position, draw_enemy_fov, minimap};
use minifb::{Window, WindowOptions, Key};
use image::GenericImageView;
use nalgebra_glm::Vec2;
use std::time::{Duration, Instant};
use std::f32::consts::PI;

fn calculate_fps(start_time: Instant, frame_count: usize) -> f64 {
    let duration = start_time.elapsed().as_secs_f64();
    frame_count as f64 / duration
}

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

    game_start(width, height, &mut framebuffer, &mut window);

}