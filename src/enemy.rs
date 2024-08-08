use nalgebra_glm::Vec2;

pub struct Enemy {
    pos: Vec2,
    dir: Vec2,
    speed: f32,
}

impl Enemy {
    pub fn new(pos: Vec2, dir: Vec2, speed: f32) -> Self {
        Enemy { pos, dir, speed}
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    pub fn set_dir(&mut self, new_dir: Vec2) {
        self.dir = new_dir;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.pos.x += self.dir.x * self.speed * delta_time;
        self.pos.y += self.dir.y * self.speed * delta_time;
    }
    
}