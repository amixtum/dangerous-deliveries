use super::obstacle::Obstacle;
use super::traversability::Traversability;
use super::player::Player;

use util::vec_ops;
use util::lsystem::{Turtle, Alphabet, LSystem};

use rand::Rng;

use std::fs;

pub struct CellTable {
    goals: Vec<(i32, i32)>,
    n_goals: u32,

    width: usize,
    height: usize,
    table: Vec<Vec<Obstacle>>,

    n_falls: u32,
    max_falls: u32,

    lsystem: LSystem,
    turtles: Vec<Turtle>,
    saved_positions: Vec<Vec<(i32, i32, i32)>>,
    lsystem_update: u32,
    pit_gen_p: f32,
    rail_gen_p: f32,
    continue_rail: bool,
    n_turtles: u32,
}

impl CellTable {
    pub fn new(width: usize, height: usize, lsystem_file: &str, turtle_file: &str) -> Self {
        let mut ct = CellTable {
            goals: Vec::new(),
            n_goals: 16,

            width,
            height,
            table: Vec::new(),

            n_falls: 0,
            max_falls: 5,

            lsystem: LSystem::from_file(lsystem_file),
            turtles: Vec::new(),
            saved_positions: Vec::new(),
            lsystem_update: 8,
            pit_gen_p: 0.1,
            rail_gen_p: 0.2,
            continue_rail: false,
            n_turtles: 8,
        };

        for x in 0..width {
            ct.table.push(Vec::new());
            for _ in 0..height {
                ct.table[x].push(Obstacle::Platform(0));
            }
        }

        ct.properties_from_file(turtle_file);

        ct.lsystem.update_n(ct.lsystem_update);

        ct.regen_table();

        ct
    }
}


impl CellTable {
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
                if words[0] == "lsystem_update" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.lsystem_update = num; 
                    }
                }
                else if words[0] == "n_turtles" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.n_turtles = num; 
                    }
                }
                else if words[0] == "pit_gen_p" {
                    if let Ok(num) = words[1].parse::<f32>() {
                        self.pit_gen_p = num; 
                    }
                }
                else if words[0] == "rail_gen_p" {
                    if let Ok(num) = words[1].parse::<f32>() {
                        self.rail_gen_p = num; 
                    }
                }
                else if words[0] == "n_goals" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.n_goals = num;
                    }
                }
                else if words[0] == "max_falls" {
                     if let Ok(num) = words[1].parse::<u32>() {
                        self.max_falls = num;
                    }                   
                }
            }
        }
    }



    pub fn inc_fallover(&mut self) {
        self.n_falls += 1;
    }

    pub fn check_falls(&mut self) -> bool {
        let b = self.n_falls >= self.max_falls;
        if b {
            self.n_falls = 0;
        }
        b
    }

    pub fn get_falls(&self) -> u32 {
        self.n_falls
    }

    pub fn max_falls(&self) -> u32 {
        self.max_falls
    }

    pub fn goals_left(&self) -> u32 {
        self.n_goals - self.goals_reached()
    }

    pub fn regen_table(&mut self) {
         for x in 0..self.width {
            for y in 0..self.height {
                self.table[x][y] = Obstacle::Platform(0);
            }
        }

        self.turtles.clear();
        self.saved_positions.clear();

        for _ in 0..self.n_turtles {
            let p_x = rand::thread_rng().gen_range(1..(self.width - 1)) as i32;
            let p_y = rand::thread_rng().gen_range(1..(self.height - 1)) as i32;
            let p_z: i32 = rand::thread_rng().gen_range(-1..=1);
            let mut d_x: i32;
            let mut d_y: i32;
            if p_x - self.width as i32 / 2 > 0 {
                d_x = -1;
            } 
            else {
                d_x = 1;
            }

            if p_x - self.height as i32 / 2 > 0 {
                d_y = -1;
            }
            else {
                d_y = 1;
            }
            while d_x == 0 && d_y == 0 {
                d_x = rand::thread_rng().gen_range(-1..=1);
                d_y = rand::thread_rng().gen_range(-1..=1);
            }
            self.turtles.push(Turtle::new((p_x as i32, p_y as i32, p_z), (d_x, d_y, 0)));
            self.saved_positions.push(Vec::new());
        }

        let mut c_idx = 0; 
        while c_idx < self.lsystem.current.len() {
            self.compute_turtles(self.lsystem.current[c_idx]);
            c_idx += 1;
        }

        self.n_falls = 0;

        self.regen_goals();
    }
    pub fn regen_goals(&mut self) {
        self.goals.clear();

        for _ in 0..self.n_goals {
            let mut p_x = rand::thread_rng().gen_range(1..(self.width - 1)) as i32;
            let mut p_y = rand::thread_rng().gen_range(1..(self.height - 1)) as i32;        

            let mut found = false;
            while !found {
                match self.get_obstacle(p_x, p_y) {
                    Obstacle::Pit => {
                        p_x = rand::thread_rng().gen_range(1..(self.width - 1)) as i32;
                        p_y = rand::thread_rng().gen_range(1..(self.height - 1)) as i32;        
                    },
                    _ => {
                        found = true;
                        self.goals.push((p_x, p_y));
                    }
                }
            }
        }
    }

    pub fn goals_reached(&self) -> u32 {
        self.n_goals - self.goals.len() as u32
    }

    pub fn get_goals(&self) -> &Vec<(i32, i32)> {
        return &self.goals;
    }

    pub fn remove_goal_if_reached(&mut self, player: &Player) -> bool {
        let mut removed = false;
        let mut to_remove = Vec::new();
        for index in (0..self.goals.len()).rev() {
            if self.goals[index].0 == player.x() && self.goals[index].1 == player.y() {
                to_remove.push(index);
            }
        }
        for index in to_remove {
            removed = true;
            self.goals.remove(index);
        }

        removed
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
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
            else if self.turtles[turtle_index].position.0 as usize >= self.width {
                self.turtles[turtle_index].direction.0 = -1;
            }

            if self.turtles[turtle_index].position.1 < 0 {
                self.turtles[turtle_index].direction.1 = 1;
            }
            else if self.turtles[turtle_index].position.1 as usize >= self.height {
                self.turtles[turtle_index].direction.1 = -1;
            }

            self.turtles[turtle_index].position.0 = self.turtles[turtle_index].position.0.clamp(0, self.width as i32 - 1);
            self.turtles[turtle_index].position.1 = self.turtles[turtle_index].position.1.clamp(0, self.height as i32 - 1);
            self.turtles[turtle_index].position.2 = self.turtles[turtle_index].position.2.clamp(-1, 1);
            while self.turtles[turtle_index].position.0 == self.width as i32 / 2 && self.turtles[turtle_index].position.1 == self.height as i32 / 2 {
                self.turtles[turtle_index].position.0 += self.turtles[turtle_index].direction.0;
                self.turtles[turtle_index].position.1 += self.turtles[turtle_index].direction.1;
                self.turtles[turtle_index].position.2 += self.turtles[turtle_index].direction.2;

                if self.turtles[turtle_index].position.0 < 0 {
                    self.turtles[turtle_index].direction.0 = 1;
                }
                else if self.turtles[turtle_index].position.0 as usize >= self.width {
                    self.turtles[turtle_index].direction.0 = -1;
                }

                if self.turtles[turtle_index].position.1 < 0 {
                    self.turtles[turtle_index].direction.1 = 1;
                }
                else if self.turtles[turtle_index].position.1 as usize >= self.height {
                    self.turtles[turtle_index].direction.1 = -1;
                }

                self.turtles[turtle_index].position.0 = self.turtles[turtle_index].position.0.clamp(0, self.width as i32 - 1);
                self.turtles[turtle_index].position.1 = self.turtles[turtle_index].position.1.clamp(0, self.height as i32 - 1);
                self.turtles[turtle_index].position.2 = self.turtles[turtle_index].position.2.clamp(-1, 1);
            }       
    }

    fn place_turtle(&mut self, turtle_index: usize) {
        if rand::thread_rng().gen_bool(self.pit_gen_p as f64) {
                self.continue_rail = false;
                self.table[self.turtles[turtle_index].position.0 as usize]
                          [self.turtles[turtle_index].position.1 as usize] = Obstacle::Pit;
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
