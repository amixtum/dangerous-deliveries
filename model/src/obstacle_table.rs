use crate::player::Player;

use super::obstacle::Obstacle;
use super::traversability::Traversability;

use std::collections::HashMap;

pub struct ObstacleTable {
    width: u32,
    height: u32,
    table: Vec<Vec<Obstacle>>,
    pub blocked: HashMap<(i32, i32), Player>,
}

impl ObstacleTable {
    pub fn new(width: u32, height: u32) -> Self {
        let mut ct = ObstacleTable {
            width,
            height,
            table: Vec::new(),
            blocked: HashMap::new(),
        };

        for x in 0..width {
            ct.table.push(Vec::new());
            for _ in 0..height {
                ct.table[x as usize].push(Obstacle::Platform);
            }
        }

        //ct.lsystem.update_n(ct.lsystem.iterations);

        ct.regen_table();

        ct
    }
}

impl ObstacleTable {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_obstacle(&self, x: i32, y: i32) -> Obstacle {
        self.table[x as usize][y as usize]
    }

    pub fn get_direction(&self, x: i32, y: i32) -> Option<(f32, f32)> {
        match self.table[x as usize][y as usize] {
            Obstacle::Platform => None,
            Obstacle::Pit => None,
            Obstacle::Rail(xdir, ydir) => Some((xdir as f32, ydir as f32)),
            Obstacle::Wall => None,
        }
    }

    pub fn can_traverse(&self, (from_x, from_y): (i32, i32), (to_x, to_y): (i32, i32)) -> bool {
        let x_diff = to_x - from_x;
        let y_diff = to_y - from_y;

        if self.blocked.contains_key(&(to_x, to_y)) && !(from_x == to_x && from_y == to_y) {
            return false;
        }

        if to_x >= 0 && to_x < self.width as i32 && to_y >= 0 && to_y < self.height as i32 {
            return x_diff.abs() <= 1 && y_diff.abs() <= 1;
        }

        false
    }

    pub fn traversability(
        &self,
        (from_x, from_y): (i32, i32),
        (to_x, to_y): (i32, i32),
    ) -> Traversability {
        let x_diff = to_x as i32 - from_x as i32;
        let y_diff = to_y as i32 - from_y as i32;

        if to_x >= 0
            && to_x < self.width as i32
            && to_y >= 0
            && to_y < self.height as i32
            && x_diff.abs() <= 1
            && y_diff.abs() <= 1
        {
            return Traversability::Flat;
        }

        return Traversability::No;
    }

    pub fn set_obstacle(&mut self, (x, y): (i32, i32), obs: Obstacle) {
        self.table[x as usize][y as usize] = obs;
    }

    // assumes an obstacle already exists at x, y and
    // and copies its height to the platform
    // if it is a pit, it gets height 0
    pub fn set_platform(&mut self, (x, y): (i32, i32)) {
        match self.table[x as usize][y as usize] {
            Obstacle::Pit => {
                self.table[x as usize][y as usize] = Obstacle::Platform;
            }
            _ => {
                self.table[x as usize][y as usize] = Obstacle::Platform;
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.table.clear();
        for x in 0..width {
            self.table.push(Vec::new());
            for _ in 0..height {
                self.table[x as usize].push(Obstacle::Platform);
            }
        }
        self.regen_table();
    }

    pub fn regen_table(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.table[x as usize][y as usize] = Obstacle::Platform;
            }
        }
    }
}
