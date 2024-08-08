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

    pub fn update(&mut self, delta_time: f32, maze: &Vec<Vec<char>>, block_size: f32) {
        let new_pos = Vec2::new(
            self.pos.x + self.dir.x * self.speed * delta_time,
            self.pos.y + self.dir.y * self.speed * delta_time,
        );
        
        let temp_enemy = Enemy {
            pos: new_pos,
            dir: self.dir,
            speed: self.speed,
        };

        if temp_enemy.check_collision_with_wall(maze, block_size) {

            self.dir.x = -self.dir.x;
            self.dir.y = -self.dir.y;
        } else {
            self.pos = new_pos;
        }
    }

    pub fn check_collision_with_wall(&self, maze: &Vec<Vec<char>>, block_size: f32) -> bool {
        let maze_x = (self.pos.x / block_size) as usize;
        let maze_y = (self.pos.y / block_size) as usize;

        maze[maze_y][maze_x] == '+' || maze[maze_y][maze_x] == '|' || maze[maze_y][maze_x] == '-'
    }
    
}