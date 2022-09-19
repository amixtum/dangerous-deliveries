use crate::player::Player;

use super::obstacle::Obstacle;
use super::traversability::Traversability;

use util::lsystem::{Alphabet, LSystem, Turtle};
use util::vec_ops;

use rand::Rng;

use std::collections::HashMap;
use std::fs;

pub struct ObstacleTable {
    width: u32,
    height: u32,
    table: Vec<Vec<Obstacle>>,
    pub blocked: HashMap<(i32, i32), Player>,

    lsystem: LSystem,
    _turtles: Vec<Turtle>,
    _saved_positions: Vec<Vec<(i32, i32, i32)>>,
    pit_gen_p: f32,
    rail_gen_p: f32,
    _continue_rail: bool,
}

impl ObstacleTable {
    pub fn new(width: u32, height: u32, lsystem_file: &str, table_file: &str) -> Self {
        let mut ct = ObstacleTable {
            width,
            height,
            table: Vec::new(),
            blocked: HashMap::new(),

            lsystem: LSystem::from_file(lsystem_file),
            _turtles: Vec::new(),
            _saved_positions: Vec::new(),
            pit_gen_p: 0.1,
            rail_gen_p: 0.2,
            _continue_rail: false,
        };

        for x in 0..width {
            ct.table.push(Vec::new());
            for _ in 0..height {
                ct.table[x as usize].push(Obstacle::Platform);
            }
        }

        ct.properties_from_file(table_file);

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

        if to_x >= 0
            && to_x < self.width as i32
            && to_y >= 0
            && to_y < self.height as i32
            && !self.blocked.contains_key(&(to_x, to_y))
        {
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

    pub fn properties_from_file(&mut self, path: &str) {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if let Some(c) = line.chars().nth(0) {
                    if c == '#' {
                        continue;
                    }
                } else {
                    continue;
                }

                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                if words[0] == "pit_gen_p" {
                    if let Ok(num) = words[1].parse::<f32>() {
                        self.pit_gen_p = num;
                    }
                } else if words[0] == "rail_gen_p" {
                    if let Ok(num) = words[1].parse::<f32>() {
                        self.rail_gen_p = num;
                    }
                }
            }
        }
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

    pub fn set_lsystem(&mut self, lsystem: LSystem) {
        self.lsystem = lsystem;
        self.lsystem.update_n(self.lsystem.iterations);
        self.regen_table();
    }

    fn _regen_turtles(&mut self) {
        self._turtles.clear();
        self._saved_positions.clear();

        let x_skip = (self.width as i32 - self.width as i32 / 4) / self.lsystem.turtles as i32;
        let y_skip = (self.height as i32 - self.height as i32 / 4) / self.lsystem.turtles as i32;

        let mut p_x = self.width as i32 / 8;
        let mut p_y = self.height as i32 / 8;
        for _ in 0..self.lsystem.turtles {
            let p_z: i32 = rand::thread_rng().gen_range(-1..=1);

            let mut d_x: i32;
            let mut d_y: i32;

            let xdiff = p_x - self.width as i32 / 2;
            let ydiff = p_y - self.height as i32 / 2;

            if xdiff > 0 {
                d_x = -1;
            } else if xdiff == 0 {
                d_x = 0;
            } else {
                d_x = 1;
            }

            if ydiff > 0 {
                d_y = 1;
            } else if xdiff == 0 {
                d_y = 0;
            } else {
                d_y = -1;
            }

            while d_x == 0 && d_y == 0 {
                d_x = rand::thread_rng().gen_range(-1..=1);
                d_y = rand::thread_rng().gen_range(-1..=1);
            }

            self._turtles
                .push(Turtle::new((p_x as i32, p_y as i32, p_z), (d_x, d_y, 0)));
            self._saved_positions.push(Vec::new());

            p_x += x_skip as i32;
            p_y += y_skip as i32;
        }
    }

    fn _compute_turtles(&mut self, letter: Alphabet) {
        let mut turtle_index = 0;
        while turtle_index < self._turtles.len() {
            match letter {
                Alphabet::Fwd => {
                    self._fwd_turtle(turtle_index);
                }
                Alphabet::Left => {
                    let direction = vec_ops::rotate_left((
                        self._turtles[turtle_index].direction.0,
                        self._turtles[turtle_index].direction.1,
                    ));
                    self._turtles[turtle_index].direction.0 = direction.0;
                    self._turtles[turtle_index].direction.1 = direction.1;
                }
                Alphabet::Right => {
                    let direction = vec_ops::rotate_right((
                        self._turtles[turtle_index].direction.0,
                        self._turtles[turtle_index].direction.1,
                    ));
                    self._turtles[turtle_index].direction.0 = direction.0;
                    self._turtles[turtle_index].direction.1 = direction.1;
                }
                Alphabet::Up => {
                    self._turtles[turtle_index].direction.2 += 1;
                }
                Alphabet::Down => {
                    self._turtles[turtle_index].direction.2 -= 1;
                }
                Alphabet::Place => {
                    self._place_turtle(turtle_index);
                }
                Alphabet::Save => {
                    self._saved_positions[turtle_index].push(self._turtles[turtle_index].position);
                }
                Alphabet::Return => {
                    if let Some(return_to) = self._saved_positions[turtle_index].pop() {
                        self._turtles[turtle_index].position = return_to;
                    }
                }
                Alphabet::None => {}
            }

            turtle_index += 1;
        }
    }

    fn _fwd_turtle(&mut self, turtle_index: usize) {
        self._turtles[turtle_index].position.0 += self._turtles[turtle_index].direction.0;
        self._turtles[turtle_index].position.1 += self._turtles[turtle_index].direction.1;
        self._turtles[turtle_index].position.2 = 0;

        if self._turtles[turtle_index].position.0 <= 0 {
            self._turtles[turtle_index].direction.0 = 1;
        } else if self._turtles[turtle_index].position.0 >= self.width as i32 - 1 {
            self._turtles[turtle_index].direction.0 = -1;
        }

        if self._turtles[turtle_index].position.1 <= 0 {
            self._turtles[turtle_index].direction.1 = 1;
        } else if self._turtles[turtle_index].position.1 >= self.height as i32 - 1 {
            self._turtles[turtle_index].direction.1 = -1;
        }

        self._turtles[turtle_index].position.0 = self._turtles[turtle_index]
            .position
            .0
            .clamp(0, self.width as i32 - 1);
        self._turtles[turtle_index].position.1 = self._turtles[turtle_index]
            .position
            .1
            .clamp(0, self.height as i32 - 1);
        self._turtles[turtle_index].position.2 = 0;
    }

    fn _place_turtle(&mut self, turtle_index: usize) {
        if rand::thread_rng().gen_bool(self.pit_gen_p as f64) {
            self._continue_rail = false;
            if !(self._turtles[turtle_index].position.0 == self.width as i32 / 2
                && self._turtles[turtle_index].position.1 == self.height as i32 / 2)
            {
                self.table[self._turtles[turtle_index].position.0 as usize]
                    [self._turtles[turtle_index].position.1 as usize] = Obstacle::Wall;
            }
        } else if self._continue_rail {
            self.table[self._turtles[turtle_index].position.0 as usize]
                [self._turtles[turtle_index].position.1 as usize] = Obstacle::Rail(
                self._turtles[turtle_index].direction.0,
                self._turtles[turtle_index].direction.1,
            );
        } else if rand::thread_rng().gen_bool(self.rail_gen_p as f64) {
            self._continue_rail = true;
            self.table[self._turtles[turtle_index].position.0 as usize]
                [self._turtles[turtle_index].position.1 as usize] = Obstacle::Rail(
                self._turtles[turtle_index].direction.0,
                self._turtles[turtle_index].direction.1,
            );
        } else {
            self.table[self._turtles[turtle_index].position.0 as usize]
                [self._turtles[turtle_index].position.1 as usize] = Obstacle::Platform;
        }
    }
}
