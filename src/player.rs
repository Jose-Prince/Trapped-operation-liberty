use nalgebra_glm::{Vec2};

pub struct Player {
    pub pos: Vec2,
}

impl Player {
    pub fn new(x : f32, y:f32) -> Self {
        Player {
            pos: Vec2::new(x,y),
        }
    }
}