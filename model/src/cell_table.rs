use super::obstacle::Obstacle;
use super::player::Player;
use super::cell::Cell;
use super::util;


pub struct GameManager {
    pub width: usize,
    pub height: usize,
    pub table: Vec<Vec<Cell>>,
    pub player: Player,
}

impl GameManager {
    pub fn new(width: usize, height: usize) -> Self {
        let mut ct = GameManager {
            width,
            height,
            table: Vec::new(),
            player: Player::new(0, 0, 0),
        };

        for x in 0..width {
            ct.table.push(Vec::new());
            for y in 0..height {
                ct.table[x].push(Cell::new(x as i32, y as i32, Obstacle::Platform(0)));
            }
        }

        // TODO: mapgen

        ct
    }
}


impl GameManager {
    pub fn get_obstacle(&self, x: i32, y: i32) -> Obstacle {
        Obstacle::clone(&self.table[x as usize][y as usize].obstacle)
    }

    pub fn get_height(&self, x: i32, y: i32) -> i32 {
        match self.table[x as usize][y as usize].obstacle {
            Obstacle::Platform(height) => height,
            Obstacle::Pit => -999, 
            Obstacle::Rail(height, ..) => height,
        }
    }

    pub fn get_direction(&self, x: i32, y: i32) -> Option<(f32, f32)> {
        match self.table[x as usize][y as usize].obstacle {
            Obstacle::Platform(_) => None,
            Obstacle::Pit => None, 
            Obstacle::Rail(_, pair) => Some(pair),
        }
    }

    pub fn can_traverse(&self, (from_x, from_y): (i32, i32),
                        (to_x, to_y): (i32, i32)) -> bool {
        let x_diff = to_x as i32 - from_x as i32; 
        let y_diff = to_y as i32 - from_y as i32;
        let h_diff = self.get_height(to_x, to_y) - self.get_height(from_x, from_y);

        x_diff.abs() <= 1 && y_diff.abs() <= 1 && h_diff.abs() <= 1 
    }

    pub fn compute_move(&mut self, 
                        speed_damp: f32, 
                        balance_damp: f32, 
                        turn_fact: f32,
                        up_speed_fact: f32,
                        down_speed_fact: f32,
                        max_speed: f32,
                        fallover_threshold: f32,
                        (inst_x, inst_y): (f32, f32)) {
        let last_speed = self.player.speed;

        // compute speed
        self.player.speed.0 = self.player.speed.0 * speed_damp + inst_x;
        self.player.speed.1 = self.player.speed.1 * speed_damp + inst_y;

        if self.player.speed.0.abs() > max_speed {
            self.player.speed.0 = max_speed;
        }
        if self.player.speed.1.abs() > max_speed {
            self.player.speed.1 = max_speed;
        }

        // compute balance
        self.player.balance.0 = self.player.balance.0 * balance_damp + (self.player.speed.0 - last_speed.0) * turn_fact;
        self.player.balance.1 = self.player.balance.1 * balance_damp + (self.player.speed.1 - last_speed.1) * turn_fact;

        if util::magnitude(self.player.balance) >= fallover_threshold {
            self.player.speed = (0.0, 0.0);
            self.player.balance = (0.0, 0.0);
        }

        // compute position
        let mut next_pos = self.player.position;

        let temp = next_pos.0 as f32 + self.player.speed.0;
        next_pos.0 += temp as i32;

        let temp = next_pos.1 as f32 + self.player.speed.1;
        next_pos.1 += temp as i32;

        if self.can_traverse((self.player.position.0, self.player.position.1), (next_pos.0, next_pos.1)) {
            if self.get_height(next_pos.0, next_pos.1) < self.get_height(self.player.position.0, self.player.position.1) {
                self.player.speed.0 *= down_speed_fact;
                self.player.speed.1 *= down_speed_fact;
            }

            else if self.get_height(next_pos.0, next_pos.1) > self.get_height(self.player.position.0, self.player.position.1) {
                self.player.speed.0 *= up_speed_fact;
                self.player.speed.1 *= up_speed_fact;
            }

        } else {
            self.player.speed = (0.0, 0.0);
            self.player.balance = (0.0, 0.0);
        }
    }
}
