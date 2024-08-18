use nalgebra_glm::Vec2;
use minifb::{Window, Key};
use std::f32::consts::PI;
use crate::AudioPlayer;
use crate::maze::is_wall;
use crate::Framebuffer;
use crate::Color;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32, // Campo de visión
    prev_mouse_x: f32,
    prev_mouse_y: f32,
    mouse_sensitivity: f32, // Sensibilidad del ratón
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32, fov: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            a,
            fov,
            prev_mouse_x: 0.0,
            prev_mouse_y: 0.0,
            mouse_sensitivity: 0.01, // Ajusta la sensibilidad del ratón según sea necesario
        }
    }

    pub fn get_pos(&mut self) -> Vec2 {
        self.pos
    } 

    pub fn get_a(&mut self) -> f32 {
        self.a
    }

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: f32, framebuffer: &mut Framebuffer, audio: &mut AudioPlayer) -> (char, Vec2) {
        const MOVE_SPEED: f32 = 5.0;
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 30.0;
        let mut key_down = '\0';
        
        let cos_a = self.a.cos();
        let sin_a = self.a.sin();
    
        // Rotación de la cámara con las teclas Left y Right
        if window.is_key_down(Key::Left) {
            self.a -= ROTATION_SPEED;
        }
        if window.is_key_down(Key::Right) {
            self.a += ROTATION_SPEED;
        }
    
        let mut move_x = 0.0;
        let mut move_y = 0.0;
    
        // Movimiento hacia adelante y hacia atrás (W y S)
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            audio.play();
            move_x += MOVE_SPEED * cos_a;
            move_y += MOVE_SPEED * sin_a;
    
            key_down = 'w';
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            move_x -= MOVE_SPEED * cos_a;
            move_y -= MOVE_SPEED * sin_a;
    
            key_down = 's'
        }
    
        // Movimiento lateral (A y D)
        if window.is_key_down(Key::A) {
            move_x += MOVE_SPEED * sin_a;
            move_y -= MOVE_SPEED * cos_a;
    
            key_down = 'a'
        }
        if window.is_key_down(Key::D) {
            move_x -= MOVE_SPEED * sin_a;
            move_y += MOVE_SPEED * cos_a;
    
            key_down = 'd'
        }
    
        // Normalizar movimiento en diagonal
        let diagonal_speed = MOVE_SPEED / (2.0f32).sqrt();
        if (window.is_key_down(Key::W) || window.is_key_down(Key::Up)) && window.is_key_down(Key::A) {
            move_x = diagonal_speed * (cos_a + sin_a);
            move_y = diagonal_speed * (sin_a - cos_a);
        }
        if (window.is_key_down(Key::W) || window.is_key_down(Key::Up)) && window.is_key_down(Key::D) {
            move_x = diagonal_speed * (cos_a - sin_a);
            move_y = diagonal_speed * (sin_a + cos_a);
        }
        if (window.is_key_down(Key::S) || window.is_key_down(Key::Down)) && window.is_key_down(Key::A) {
            move_x = diagonal_speed * (-cos_a + sin_a);
            move_y = diagonal_speed * (-sin_a - cos_a);
        }
        if (window.is_key_down(Key::S) || window.is_key_down(Key::Down)) && window.is_key_down(Key::D) {
            move_x = diagonal_speed * (-cos_a - sin_a);
            move_y = diagonal_speed * (-sin_a + cos_a);
        }
    
        let new_pos = Vec2::new(self.pos.x + move_x, self.pos.y + move_y);
        if !is_wall(maze, (new_pos.x / block_size as f32) as usize, (new_pos.y / block_size as f32) as usize) {
            self.pos = new_pos;
            return (key_down, self.pos);
        } else {
            return ('\0', self.pos);
        }
    }
        

    pub fn update_mouse(&mut self, mouse_x: f32, mouse_y: f32, window_width: f32, window_height: f32) {
        // Calcula el movimiento del ratón
        let delta_x = mouse_x - self.prev_mouse_x;
        let delta_y = mouse_y - self.prev_mouse_y;
    
        // Actualiza el ángulo del jugador basado en el movimiento del ratón
        // Invertir el signo aquí para que el movimiento del ratón a la derecha mueva la cámara a la derecha
        self.a += delta_x * self.mouse_sensitivity;
    
        // Asegúrate de que el ángulo esté en el rango [0, 2π)
        self.a = self.a.rem_euclid(2.0 * PI);
    
        // Actualiza la posición previa del ratón
        self.prev_mouse_x = mouse_x;
        self.prev_mouse_y = mouse_y;
    
        // Opcional: Mueve el cursor al centro de la ventana
        // Esto puede ser necesario para obtener movimientos continuos
        // window.set_cursor_pos((window_width / 2.0) as usize, (window_height / 2.0) as usize).unwrap();
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