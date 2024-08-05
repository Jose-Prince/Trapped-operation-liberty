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
        const MOVE_SPEED: f32 = 10.0;
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;

        let cos_a = self.a.cos();
        let sin_a = self.a.sin();

        if window.is_key_down(Key::Left) {
            self.a -= ROTATION_SPEED;
        }
        if window.is_key_down(Key::Right) {
            self.a += ROTATION_SPEED;
        }
        if window.is_key_down(Key::Up) {
            let move_x = MOVE_SPEED * cos_a;
            let move_y = MOVE_SPEED * sin_a;
            let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
            let angle = (move_y.atan2(move_x) - self.a).to_radians();
            let angle = normalize_angle(angle);

            if let Some(intersect) = cast_ray(&new_pos, angle, maze, block_size, false, None) {
                if !is_wall(maze, (intersect.x / block_size as f32) as usize, (intersect.y / block_size as f32) as usize) {
                    self.pos = new_pos;
                }
            }
        }
        if window.is_key_down(Key::Down) {
            let move_x = -MOVE_SPEED * cos_a;
            let move_y = -MOVE_SPEED * sin_a;
            let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
            let angle = (move_y.atan2(move_x) - self.a).to_radians();
            let angle = normalize_angle(angle);

            if let Some(intersect) = cast_ray(&new_pos, angle, maze, block_size, false, None) {
                if !is_wall(maze, (intersect.x / block_size as f32) as usize, (intersect.y / block_size as f32) as usize) {
                    self.pos = new_pos;
                }
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
