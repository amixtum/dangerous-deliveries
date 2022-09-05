use super::obstacle::Obstacle;
use super::traversability::Traversability;

use util::vec_ops;
use util::lsystem::{Turtle, Alphabet, LSystem};

use rand::Rng;

use std::fs;

pub struct ObstacleTable {
    width: u32,
    height: u32,
    table: Vec<Vec<Obstacle>>,

    lsystem: LSystem,
    turtles: Vec<Turtle>,
    saved_positions: Vec<Vec<(i32, i32, i32)>>,
    pit_gen_p: f32,
    rail_gen_p: f32,
    continue_rail: bool,
}

impl ObstacleTable {
    pub fn new(width: u32, height: u32, lsystem_file: &str, table_file: &str) -> Self {
        let mut ct = ObstacleTable {
            width,
            height,
            table: Vec::new(),

            lsystem: LSystem::from_file(lsystem_file),
            turtles: Vec::new(),
            saved_positions: Vec::new(),
            pit_gen_p: 0.1,
            rail_gen_p: 0.2,
            continue_rail: false,
        };

        for x in 0..width {
            ct.table.push(Vec::new());
            for _ in 0..height {
                ct.table[x as usize].push(Obstacle::Platform(0));
            }
        }

        ct.properties_from_file(table_file);

        ct.lsystem.update_n(ct.lsystem.iterations);
        ct.regen_table();

        ct
    }
}


impl ObstacleTable {
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
                }
                else if words[0] == "rail_gen_p" {
                    if let Ok(num) = words[1].parse::<f32>() {
                        self.rail_gen_p = num; 
                    }
                }
            }
        }
    }

    pub fn set_lsystem(&mut self, lsystem: LSystem) {
        self.lsystem = lsystem;
        self.lsystem.update_n(self.lsystem.iterations);
        self.regen_table();
    }

    pub fn set_obstacle(&mut self, (x, y): (i32, i32), obs: Obstacle) {
        self.table[x as usize][y as usize] = obs;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width; 
        self.height = height;
        self.regen_table();
    }

    pub fn regen_table(&mut self) {
         for x in 0..self.width {
            for y in 0..self.height {
                self.table[x as usize][y as usize] = Obstacle::Platform(0);
            }
        }

        self.turtles.clear();
        self.saved_positions.clear();

        let x_skip = self.width / self.lsystem.turtles;
        let y_skip = self.height / self.lsystem.turtles; 
        let mut p_x = 0;
        let mut p_y = 0;
        for _ in 0..self.lsystem.turtles {
            let p_z: i32 = rand::thread_rng().gen_range(-1..=1);

            let d_x = rand::thread_rng().gen_range(-1..=1);
            let d_y = rand::thread_rng().gen_range(-1..=1);
            self.turtles.push(Turtle::new((p_x as i32, p_y as i32, p_z), (d_x, d_y, 0)));
            self.saved_positions.push(Vec::new());

            p_x += x_skip as i32;
            p_y += y_skip as i32;
        }

        let mut c_idx = 0; 
        while c_idx < self.lsystem.get_current().len() {
            self.compute_turtles(self.lsystem.get_current()[c_idx]);
            c_idx += 1;
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_obstacle(&self, x: i32, y: i32) -> Obstacle {
        Obstacle::clone(&self.table[x as usize][y as usize])
    }

    pub fn get_height(&self, x: i32, y: i32) -> i32 {
        match self.table[x as usize][y as usize] {
            Obstacle::Platform(height) => height,
            Obstacle::Pit => -999, 
            Obstacle::Rail(height, ..) => height,
        }
    }

    pub fn get_direction(&self, x: i32, y: i32) -> Option<(f32, f32)> {
        match self.table[x as usize][y as usize] {
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

    pub fn traversability(&self, 
                          (from_x, from_y): (i32, i32),
                          (to_x, to_y): (i32, i32)) -> Traversability {
        let x_diff = to_x as i32 - from_x as i32; 
        let y_diff = to_y as i32 - from_y as i32;
        let h_diff = self.get_height(to_x, to_y) - self.get_height(from_x, from_y);

        if x_diff.abs() <= 1 && y_diff.abs() <= 1 && h_diff.abs() <= 1 {
            if h_diff > 0 {
                return Traversability::Up;
            }
            else if h_diff < 0 {
                return Traversability::Down;
            }
            else {
                return Traversability::Flat;
            }
        }

        return Traversability::No;
    }

    fn compute_turtles(&mut self, letter: Alphabet) {
        let mut turtle_index = 0;
        while turtle_index < self.turtles.len() {
            match letter {
                Alphabet::Fwd => {
                    self.fwd_turtle(turtle_index);
                }
                Alphabet::Left => {
                    let direction = vec_ops::rotate_left((self.turtles[turtle_index].direction.0, self.turtles[turtle_index].direction.1));
                    self.turtles[turtle_index].direction.0 = direction.0;
                    self.turtles[turtle_index].direction.1 = direction.1;
                }
                Alphabet::Right => {
                    let direction = vec_ops::rotate_right((self.turtles[turtle_index].direction.0, self.turtles[turtle_index].direction.1));
                    self.turtles[turtle_index].direction.0 = direction.0;
                    self.turtles[turtle_index].direction.1 = direction.1;
                }
                Alphabet::Up => {
                    self.turtles[turtle_index].direction.2 += 1;
                }
                Alphabet::Down => {
                    self.turtles[turtle_index].direction.2 -= 1;
                }
                Alphabet::Place => {
                    self.place_turtle(turtle_index);
                }
                Alphabet::Save => {
                    self.saved_positions[turtle_index].push(self.turtles[turtle_index].position);
                }
                Alphabet::Return => {
                    if let Some(return_to) = self.saved_positions[turtle_index].pop() {
                        self.turtles[turtle_index].position = return_to;
                    }
                }
                Alphabet::None => {
                }
            }

            turtle_index += 1;
        }

    }

    fn fwd_turtle(&mut self, turtle_index: usize) {
            self.turtles[turtle_index].position.0 += self.turtles[turtle_index].direction.0;
            self.turtles[turtle_index].position.1 += self.turtles[turtle_index].direction.1;
            self.turtles[turtle_index].position.2 += self.turtles[turtle_index].direction.2;

            if self.turtles[turtle_index].position.0 < 0 {
                self.turtles[turtle_index].direction.0 = 1;
            }
            else if self.turtles[turtle_index].position.0 >= self.width as i32 {
                self.turtles[turtle_index].direction.0 = -1;
            }

            if self.turtles[turtle_index].position.1 < 0 {
                self.turtles[turtle_index].direction.1 = 1;
            }
            else if self.turtles[turtle_index].position.1 >= self.height as i32{
                self.turtles[turtle_index].direction.1 = -1;
            }

            self.turtles[turtle_index].position.0 = self.turtles[turtle_index].position.0.clamp(0, self.width as i32 - 1);
            self.turtles[turtle_index].position.1 = self.turtles[turtle_index].position.1.clamp(0, self.height as i32 - 1);
            self.turtles[turtle_index].position.2 = self.turtles[turtle_index].position.2.clamp(-1, 1);
    }

    fn place_turtle(&mut self, turtle_index: usize) {
        if rand::thread_rng().gen_bool(self.pit_gen_p as f64) {
                self.continue_rail = false;
                if !(self.turtles[turtle_index].position.0 == self.width as i32 / 2 && self.turtles[turtle_index].position.1 == self.height as i32 / 2){
                    self.table[self.turtles[turtle_index].position.0 as usize]
                              [self.turtles[turtle_index].position.1 as usize] = Obstacle::Pit;
                }
            }
            else if self.continue_rail {
                self.table[self.turtles[turtle_index].position.0 as usize]
                          [self.turtles[turtle_index].position.1 as usize] = Obstacle::Rail(self.turtles[turtle_index].position.2, 
                                                                                            (self.turtles[turtle_index].direction.0 as f32,
                                                                                             self.turtles[turtle_index].direction.1 as f32));
            }
            else if rand::thread_rng().gen_bool(self.rail_gen_p as f64) {
                self.continue_rail = true;
                self.table[self.turtles[turtle_index].position.0 as usize]
                          [self.turtles[turtle_index].position.1 as usize] = Obstacle::Rail(self.turtles[turtle_index].position.2, 
                                                                                            (self.turtles[turtle_index].direction.0 as f32,
                                                                                             self.turtles[turtle_index].direction.1 as f32));
            }
            else {
                self.table[self.turtles[turtle_index].position.0 as usize]
                          [self.turtles[turtle_index].position.1 as usize] = Obstacle::Platform(self.turtles[turtle_index].position.2);
            }
    }
}
