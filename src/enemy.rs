use nalgebra_glm::Vec2;

pub struct Enemy {
    pos: Vec2,
    a: f32,
    speed: f32,
    fov_angle: f32,
    fov_range: f32
}

impl Enemy {
    pub fn new(pos: Vec2, a: f32, speed: f32, fov_angle: f32, fov_range: f32) -> Self {
        Enemy { pos, a, speed, fov_angle, fov_range}
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn get_a(&self) -> f32 {
        self.a
    }

    pub fn get_fov_angle(&self) -> f32 {
        self.fov_angle
    }

    pub fn get_fov_range(&self) -> f32 {
        self.fov_range
    }

    pub fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    pub fn update(&mut self, delta_time: f32, maze: &Vec<Vec<char>>, block_size: f32) -> bool {
        let new_pos = Vec2::new(
            self.pos.x + self.a.cos() * self.speed * delta_time,
            self.pos.y + self.a.sin() * self.speed * delta_time,
        );
        
        let temp_enemy = Enemy {
            pos: new_pos,
            a: self.a,
            speed: self.speed,
            fov_angle: self.fov_angle,
            fov_range: self.fov_range,
        };

        if temp_enemy.check_collision_with_wall(maze, block_size) {
            self.a = -self.a;
        } else {
            self.pos = new_pos;
        }

        if temp_enemy.check_collision_with_player(maze, block_size) {
            return true;
        } else {
            return false;
        }
    }

    pub fn check_collision_with_wall(&self, maze: &Vec<Vec<char>>, block_size: f32) -> bool {
        let maze_x = (self.pos.x / block_size) as usize;
        let maze_y = (self.pos.y / block_size) as usize;

        maze[maze_y][maze_x] == '+' || maze[maze_y][maze_x] == '|' || maze[maze_y][maze_x] == '-' || maze[maze_y][maze_x] == '/' || maze[maze_y][maze_x] == '!'
    }

    pub fn check_collision_with_player(&self, maze: &Vec<Vec<char>>, block_size: f32) -> bool {
        let cos_a = self.a.cos().round();
        let sin_a = self.a.sin().round();
    
        let maze_x = (self.pos.x / block_size).floor() as isize;
        let maze_y = (self.pos.y / block_size).floor() as isize;
    
        let lookahead_x = maze_x - cos_a as isize;
        let lookahead_y = maze_y - sin_a as isize;
    
        // Verifica que los índices estén dentro de los límites
        if maze_y >= 0 && maze_x >= 0 && (maze_y as usize) < maze.len() && (maze_x as usize) < maze[0].len() {    
            if lookahead_y >= 0 && lookahead_x >= 0 && (lookahead_y as usize) < maze.len() && (lookahead_x as usize) < maze[0].len() {    
                return maze[maze_y as usize][maze_x as usize] == 'p' || maze[lookahead_y as usize][lookahead_x as usize] == 'p';
            }
        }
    
        false
    }
    
    
}