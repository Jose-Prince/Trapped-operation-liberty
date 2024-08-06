use nalgebra_glm::Vec2;
use minifb::{Window, Key};
use std::f32::consts::PI;
use crate::maze::is_wall;
use crate::Framebuffer;
use crate::Color;
use crate::cast_ray::{cast_ray, normalize_angle};

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

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: usize, framebuffer: &mut Framebuffer) {
        const MOVE_SPEED: f32 = 5.0;
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 30.0;
    
        let cos_a = self.a.cos();
        let sin_a = self.a.sin();
    
        // Rotación de la cámara con las teclas Left y Right
        if window.is_key_down(Key::Left) {
            self.a -= ROTATION_SPEED;
        }
        if window.is_key_down(Key::Right) {
            self.a += ROTATION_SPEED;
        }
    
        // Movimiento hacia adelante y hacia atrás (W y S)
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            let move_x = MOVE_SPEED * cos_a;
            let move_y = MOVE_SPEED * sin_a;
            let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
    
            if !is_wall(maze, (new_pos.x / block_size as f32) as usize, (new_pos.y / block_size as f32) as usize) {
                self.pos = new_pos;
            }
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            let move_x = -MOVE_SPEED * cos_a;
            let move_y = -MOVE_SPEED * sin_a;
            let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
    
            if !is_wall(maze, (new_pos.x / block_size as f32) as usize, (new_pos.y / block_size as f32) as usize) {
                self.pos = new_pos;
            }
        }
    
        if window.is_key_down(Key::A) {
            let move_x = MOVE_SPEED * sin_a;
            let move_y = -MOVE_SPEED * cos_a;
            let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
    
            if !is_wall(maze, (new_pos.x / block_size as f32) as usize, (new_pos.y / block_size as f32) as usize) {
                self.pos = new_pos;
            }
        }
        if window.is_key_down(Key::D) {
            let move_x = -MOVE_SPEED * sin_a;
            let move_y = MOVE_SPEED * cos_a;
            let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
    
            if !is_wall(maze, (new_pos.x / block_size as f32) as usize, (new_pos.y / block_size as f32) as usize) {
                self.pos = new_pos;
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
