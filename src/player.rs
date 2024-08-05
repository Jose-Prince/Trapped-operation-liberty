use nalgebra_glm::Vec2;
use minifb::{Window, Key};
use std::f32::consts::PI;
use crate::maze::is_wall;
use crate::Framebuffer;
use crate::Color;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32, // Campo de visión
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32, fov: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            a,
            fov,
        }
    }

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
        const MOVE_SPEED: f32 = 10.0;
        const ROTATION_SPEED: f32 = PI / 10.0;

        let cos_a = self.a.cos();
        let sin_a = self.a.sin();

        if window.is_key_down(Key::Left) {
            self.a -= ROTATION_SPEED;
        }
        if window.is_key_down(Key::Right) {
            self.a += ROTATION_SPEED;
        }
        if window.is_key_down(Key::Up) {
            let new_x = self.pos.x + cos_a * MOVE_SPEED;
            let new_y = self.pos.y + sin_a * MOVE_SPEED;

            if !is_wall(maze, (new_x / block_size as f32) as usize, (new_y / block_size as f32) as usize) {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }
        if window.is_key_down(Key::Down) {
            let new_x = self.pos.x - cos_a * MOVE_SPEED;
            let new_y = self.pos.y - sin_a * MOVE_SPEED;

            if !is_wall(maze, (new_x / block_size as f32) as usize, (new_y / block_size as f32) as usize) {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        const PLAYER_SIZE: usize = 5;
        framebuffer.set_current_color(Color::new(0, 255, 0));

        for y in -(PLAYER_SIZE as isize)..=(PLAYER_SIZE as isize) {
            for x in -(PLAYER_SIZE as isize)..=(PLAYER_SIZE as isize) {
                framebuffer.point((self.pos.x as isize + x), (self.pos.y as isize + y));
            }
        }
    }
}