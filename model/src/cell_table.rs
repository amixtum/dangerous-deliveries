use super::obstacle::Obstacle;
use super::traversability::Traversability;
use super::player::Player;
use super::player_event::PlayerEvent;

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

    pub n_falls: u32,
    max_falls: u32,

    // TODO Config file for turtle
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

        for _ in 0..ct.n_turtles {
            let p_x = rand::thread_rng().gen_range(0..width);
            let p_y = rand::thread_rng().gen_range(0..height);
            let p_z: i32 = rand::thread_rng().gen_range(-1..=1);
            let mut d_x: i32 = rand::thread_rng().gen_range(-1..=1);
            let mut d_y: i32 = rand::thread_rng().gen_range(-1..=1);
            while d_x == 0 && d_x == d_y {
                d_x = rand::thread_rng().gen_range(-1..=1);
                d_y = rand::thread_rng().gen_range(-1..=1);
            }
            ct.turtles.push(Turtle::new((p_x as i32, p_y as i32, p_z), (d_x, d_y, 0)));
            ct.saved_positions.push(Vec::new());
        }

        let mut c_idx = 0; 
        while c_idx < ct.lsystem.current.len() {
            ct.compute_turtles(ct.lsystem.current[c_idx]);
            c_idx += 1;
        }

        for _ in 0..ct.n_goals {
            let mut p_x = rand::thread_rng().gen_range(0..width) as i32;
            let mut p_y = rand::thread_rng().gen_range(0..height) as i32;        

            let mut found = false;
            while !found {
                match ct.get_obstacle(p_x, p_y) {
                    Obstacle::Pit => {
                        p_x = rand::thread_rng().gen_range(0..width) as i32;
                        p_y = rand::thread_rng().gen_range(0..height) as i32;        
                    },
                    _ => {
                        found = true;
                        ct.goals.push((p_x, p_y));
                    }
                }
            }
            
        }

        ct
    }
}


impl CellTable {
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
            let p_x = rand::thread_rng().gen_range(0..self.width) as i32;
            let p_y = rand::thread_rng().gen_range(0..self.height) as i32;
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
            let mut p_x = rand::thread_rng().gen_range(0..self.width) as i32;
            let mut p_y = rand::thread_rng().gen_range(0..self.height) as i32;        

            let mut found = false;
            while !found {
                match self.get_obstacle(p_x, p_y) {
                    Obstacle::Pit => {
                        p_x = rand::thread_rng().gen_range(0..self.width) as i32;
                        p_y = rand::thread_rng().gen_range(0..self.height) as i32;        
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

    pub fn compute_turtles(&mut self, letter: Alphabet) {
        let mut turtle_index = 0;
        while turtle_index < self.turtles.len() {
            match letter {
                Alphabet::Fwd => {
                    self.turtles[turtle_index].position.0 += self.turtles[turtle_index].direction.0;
                    self.turtles[turtle_index].position.1 += self.turtles[turtle_index].direction.1;
                    self.turtles[turtle_index].position.2 += self.turtles[turtle_index].direction.2;

                    if self.turtles[turtle_index].position.0 < 0 {
                        self.turtles[turtle_index].direction.0 = 1;
                    }
                    else if self.turtles[turtle_index].position.0 as usize >= self.width {
                        self.turtles[turtle_index].direction.0 = -1;
                    }
                    else if self.turtles[turtle_index].position.1 < 0 {
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].position.1 as usize >= self.height {
                        self.turtles[turtle_index].direction.0 = -1;
                    }

                    self.turtles[turtle_index].position.0 = self.turtles[turtle_index].position.0.clamp(0, self.width as i32 - 1);
                    self.turtles[turtle_index].position.1 = self.turtles[turtle_index].position.1.clamp(0, self.height as i32 - 1);
                    self.turtles[turtle_index].position.2 = self.turtles[turtle_index].position.2.clamp(-1, 1);
                }
                Alphabet::Left => {
                    let direction = vec_ops::rotate_left((self.turtles[turtle_index].direction.0, self.turtles[turtle_index].direction.1));
                    self.turtles[turtle_index].direction.0 = direction.0;
                    self.turtles[turtle_index].direction.1 = direction.1;
                    /*
                    if self.turtles[turtle_index].direction.0 == 1 && self.turtles[turtle_index].direction.1 == 0 {
                        self.turtles[turtle_index].direction.0 = 1;
                        self.turtles[turtle_index].direction.1 = -1;
                    }
                    else if self.turtles[turtle_index].direction.0 == 1 && self.turtles[turtle_index].direction.1 == 1 {
                        self.turtles[turtle_index].direction.0 = 1;
                        self.turtles[turtle_index].direction.1 = 0;
                    }
                    else if self.turtles[turtle_index].direction.0 == 1 && self.turtles[turtle_index].direction.1 == -1 {
                        self.turtles[turtle_index].direction.0 = 0;
                        self.turtles[turtle_index].direction.1 = -1;
                    }
                    else if self.turtles[turtle_index].direction.0 == -1 && self.turtles[turtle_index].direction.1 == 0 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].direction.0 == -1 && self.turtles[turtle_index].direction.1 == 1 {
                        self.turtles[turtle_index].direction.0 = 0;
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].direction.0 == -1 && self.turtles[turtle_index].direction.1 == -1 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = 0;
                    }
                    else if self.turtles[turtle_index].direction.0 == 0 && self.turtles[turtle_index].direction.1 == 1 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].direction.0 == 0 && self.turtles[turtle_index].direction.1 == -1 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = -1;
                    }
                    */
                }
                Alphabet::Right => {
                    let direction = vec_ops::rotate_right((self.turtles[turtle_index].direction.0, self.turtles[turtle_index].direction.1));
                    self.turtles[turtle_index].direction.0 = direction.0;
                    self.turtles[turtle_index].direction.1 = direction.1;
                    /*
                     if self.turtles[turtle_index].direction.0 == 1 && self.turtles[turtle_index].direction.1 == 0 {
                        self.turtles[turtle_index].direction.0 = 1;
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].direction.0 == 1 && self.turtles[turtle_index].direction.1 == 1 {
                        self.turtles[turtle_index].direction.0 = 0;
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].direction.0 == 1 && self.turtles[turtle_index].direction.1 == -1 {
                        self.turtles[turtle_index].direction.0 = 1;
                        self.turtles[turtle_index].direction.1 = 0;
                    }
                    else if self.turtles[turtle_index].direction.0 == -1 && self.turtles[turtle_index].direction.1 == 0 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = -1;
                    }
                    else if self.turtles[turtle_index].direction.0 == -1 && self.turtles[turtle_index].direction.1 == 1 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = 0;
                    }
                    else if self.turtles[turtle_index].direction.0 == -1 && self.turtles[turtle_index].direction.1 == -1 {
                        self.turtles[turtle_index].direction.0 = 0;
                        self.turtles[turtle_index].direction.1 = -1;
                    }
                    else if self.turtles[turtle_index].direction.0 == 0 && self.turtles[turtle_index].direction.1 == 1 {
                        self.turtles[turtle_index].direction.0 = -1;
                        self.turtles[turtle_index].direction.1 = 1;
                    }
                    else if self.turtles[turtle_index].direction.0 == 0 && self.turtles[turtle_index].direction.1 == -1 {
                        self.turtles[turtle_index].direction.0 = 1;
                        self.turtles[turtle_index].direction.1 = -1;
                    }                   
                    */
                }
                Alphabet::Up => {
                    self.turtles[turtle_index].direction.2 += 1;
                }
                Alphabet::Down => {
                    self.turtles[turtle_index].direction.2 -= 1;
                }
                Alphabet::Place => {
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

    pub fn reset_player(&self, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.position = (self.width as i32 / 2, self.height as i32 / 2, self.get_height(self.width as i32 / 2, self.height as i32 / 2));
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.time = 0.0;
        clone.recent_event = PlayerEvent::GameOver;
        clone
    }

    

    pub fn compute_move(&self, 
                        player: &Player,
                        (inst_x, inst_y): (f32, f32),
                        speed_damp: f32, 
                        balance_damp: f32, 
                        turn_fact: f32,
                        onrail_balance_fact: f32,
                        offrail_balance_fact: f32,
                        up_speed_fact: f32,
                        down_speed_fact: f32,
                        max_speed: f32,
                        fallover_threshold: f32,) -> (Player, PlayerEvent) {

        let mut p_event: PlayerEvent;

        // compute initial speed and balance values
        let mut player = self.compute_initial_speed_balance(player, (inst_x, inst_y), max_speed, speed_damp, balance_damp, turn_fact);

        // compute position and updated player fields
        let result = self.compute_next_position(&player, (inst_x, inst_y), onrail_balance_fact, offrail_balance_fact);
        player = result.0;
        p_event = result.1;
        let next_pos = result.2;

        player.time += 1.0 / (1.0 + vec_ops::magnitude(player.speed));

        // fallover if the player is off balance
        if player.balance.0.abs() >= fallover_threshold || player.balance.1.abs() >= fallover_threshold {
            p_event = PlayerEvent::FallOver;
            player = self.fallover(&player);
        }

        // fall into a pit. Game Over
        if let Obstacle::Pit = self.get_obstacle(next_pos.0, next_pos.1) {
            // reset the player
            return (self.reset_player(&player), PlayerEvent::GameOver);
        }

        match p_event {
            PlayerEvent::FallOver => {
                return (player, p_event);
            },
            _ => {}
        }

        // try to move player to next_pos
        // updating speed values depending on the change
        // in height after the move
        // and return the updated Player
        let traverse = self.try_traverse(&player, next_pos, up_speed_fact, down_speed_fact);
        match traverse.1 {
            PlayerEvent::FallOver => {
                return (traverse.0, traverse.1);
            },
            _ => (traverse.0, p_event),
        }
    }

    fn fallover(&self, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.recent_event = PlayerEvent::FallOver;
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);

        match self.get_obstacle(player.x(), player.y()) {
            Obstacle::Rail(height, _) => {
                let neighbors = vec_ops::neighbors(player.xy(), 
                                                (0, 0), 
                                                (self.width as i32 - 1, self.height as i32 - 1));
                let mut found = false;

                for neighbor in neighbors {
                    match self.get_obstacle(neighbor.0, neighbor.1) {
                        Obstacle::Platform(neighbor_height) => {
                            if (height - neighbor_height).abs() <= 1 {
                                clone.position = (neighbor.0, neighbor.1, neighbor_height);
                                found = true;
                                break;
                            } 

                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                if !found {
                    clone = self.reset_player(player);
                }
            },
            _ => { } 
        }

        clone
    }

    fn compute_initial_speed_balance(&self, 
                                     player: &Player, 
                                     (inst_x, inst_y): (f32, f32), 
                                     max_speed: f32, 
                                     speed_damp: f32, 
                                     balance_damp: f32, 
                                     turn_fact: f32) -> Player {
        let last_speed = player.speed;
        let last_obstacle = self.get_obstacle(player.position.0, 
                                              player.position.1);

        let mut clone = Player::clone(player);

        match last_obstacle {
            Obstacle::Platform(_) => {
                // compute speed
                clone.speed.0 = clone.speed.0 * speed_damp + inst_x;
                clone.speed.1 = clone.speed.1 * speed_damp + inst_y;

                if clone.speed.0.abs() > max_speed {
                    if clone.speed.0 < 0.0 {
                         clone.speed.0 = -max_speed;
                    }
                    clone.speed.0 = max_speed;
                }
                if clone.speed.1.abs() > max_speed {
                    if clone.speed.1 < 0.0 {
                         clone.speed.1 = -max_speed;
                    }
                    clone.speed.1 = max_speed;
                }

                // compute balance
                clone.balance.0 = clone.balance.0 * balance_damp + 
                                        (clone.speed.0 - last_speed.0) * turn_fact;

                clone.balance.1 = clone.balance.1 * balance_damp + 
                                        (clone.speed.1 - last_speed.1) * turn_fact;
            },
            Obstacle::Rail(_, (x_dir, y_dir)) => {
                // compute speed
                clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;
                clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;

                // slow down some more if the passed instantaneous velocity
                // is in the opposite direction of the rail direction
                if inst_x > 0.0 && x_dir < 0.0 && inst_y > 0.0 && y_dir < 0.0 {
                     clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                     clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;
                } 
                else if inst_x < 0.0 && x_dir > 0.0 && inst_y < 0.0 && y_dir > 0.0 {
                     clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;
                     clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;
                }
                else if  inst_x < 0.0 && x_dir > 0.0 && inst_y > 0.0 && y_dir < 0.0 {
                     clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;
                     clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                }
                else if  inst_x > 0.0 && x_dir < 0.0 && inst_y > 0.0 && y_dir < 0.0 {
                     clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;
                     clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_x > 0.0 && x_dir < 0.0 && y_dir.abs() < 0.01 {
                    clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_y > 0.0 && y_dir < 0.0 && x_dir.abs() < 0.01 {
                    clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_x < 0.0 && x_dir > 0.0 && y_dir.abs() < 0.01 {
                    clone.speed.0 = x_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_y < 0.0 && y_dir > 0.0 && x_dir.abs() < 0.01 {
                    clone.speed.1 = y_dir * vec_ops::magnitude(clone.speed) * speed_damp;                    
                }

                clone.speed.0 = clone.speed.0.clamp(-max_speed, max_speed);
                clone.speed.1 = clone.speed.1.clamp(-max_speed, max_speed);

                // compute balance
                clone.balance.0 = clone.balance.0 * balance_damp;
                clone.balance.1 = clone.balance.1 * balance_damp;
            }
            _ => { }
        }

        clone
    }

    // updated a player's balance so it must return a new player as well
    fn compute_onrail(&self, player: &Player, (inst_x, inst_y): (f32, f32), (x_dir, y_dir): (f32, f32), onrail_balance_fact: f32) -> (Player, (i32, i32, i32)) {
        let (unit_x, unit_y) = vec_ops::discrete_jmp((inst_x, inst_y));
        let mut next_pos = player.position;
        next_pos.0 = player.position.0 + unit_x;
        next_pos.1 = player.position.1 + unit_y;
        next_pos.2 = self.get_height(next_pos.0, next_pos.1);

        let mut clone = Player::clone(player);

        if clone.balance.0 >= clone.balance.1 {
            clone.balance.0 += (clone.speed.0 - (x_dir * vec_ops::magnitude(clone.speed))) * onrail_balance_fact;
        } 
        else {
            clone.balance.1 += (clone.speed.1 - (y_dir * vec_ops::magnitude(clone.speed))) * onrail_balance_fact;
        }

        (clone, next_pos)
    }

    fn compute_continue(&self, player: &Player) -> (i32, i32, i32) {
        let mut next_pos = player.position;
        let temp = player.speed.0;
        next_pos.0 = (next_pos.0 + temp as i32).clamp(next_pos.0 - 1, next_pos.0 + 1);
        next_pos.0 = next_pos.0.clamp(0, self.width as i32);

        let temp = player.speed.1;
        next_pos.1 = (next_pos.1 + temp as i32).clamp(next_pos.1 - 1, next_pos.1 + 1);
        next_pos.1 = next_pos.1.clamp(0, self.height as i32);

        next_pos.2 = self.get_height(next_pos.0, next_pos.1);

        next_pos
    }

    // Note: Must call compute_intitial_speed_balance first in order to update speed and balance
    // values, otherwise, this will compute the next position without taking into account user
    // input
    fn compute_next_position(&self, player: &Player, (inst_x, inst_y): (f32, f32), onrail_balance_fact: f32, offrail_balance_fact: f32) -> (Player, PlayerEvent, (i32, i32, i32)) {
        let mut next_pos = player.position;
        let last_obstacle = self.get_obstacle(player.x(), player.y());
        let units = vec_ops::discrete_jmp((inst_x, inst_y));
        let unit_x = units.0;
        let unit_y = units.1;

        let mut clone = Player::clone(player);
        let mut p_event = PlayerEvent::Move;

        // bump into border
        if player.x() + unit_x >= self.width as i32 ||
           player.x() + unit_x < 0 ||
           player.y() + unit_y >= self.height as i32 ||
           player.y() + unit_y < 0 {
            clone = self.fallover(player);
            return (clone, PlayerEvent::FallOver, clone.position)
        }

        // compute position
        let obs_at_next = self.get_obstacle(player.x() + unit_x, player.y() + unit_y);
        match obs_at_next {
            Obstacle::Rail(height, (x_dir, y_dir)) => {
                match last_obstacle {
                    Obstacle::Rail(last_height, (last_x_dir, last_y_dir))=> {
                        let last_units = util::vec_ops::discrete_jmp((last_x_dir, last_y_dir));
                        let cur_units = util::vec_ops::discrete_jmp((x_dir, y_dir));
                        if height > last_height {
                            clone = self.fallover(player);
                            p_event = PlayerEvent::FallOver;
                        }
                        else if cur_units.0 != last_units.0 || cur_units.1 != last_units.1 {
                            let result = self.compute_onrail(player, (inst_x, inst_y), (x_dir, y_dir), onrail_balance_fact);
                            clone = result.0;
                            next_pos = result.1;
                            p_event = PlayerEvent::OffRail;
                        } 
                        else {
                            next_pos = self.compute_continue(player);
                        }
                    },
                    _ => {
                        let result = self.compute_onrail(player, (inst_x, inst_y), (x_dir, y_dir), onrail_balance_fact);
                        clone = result.0;
                        next_pos = result.1;
                        p_event = PlayerEvent::OnRail;
                    },
                }
            },
            _ => {
                match last_obstacle {
                    Obstacle::Rail(height, _) => {
                        next_pos = (player.x() + unit_x, 
                                    player.y() + unit_y, 
                                    height);
                        clone.balance.0 += inst_x * offrail_balance_fact;
                        clone.balance.1 += inst_y * offrail_balance_fact;
                        p_event = PlayerEvent::OffRail;
                    },
                    _ => {
                        next_pos = self.compute_continue(player);
                    },
                }
            }
        }

        if next_pos.0 == player.x() && next_pos.1 == player.y() {
            clone.recent_event = PlayerEvent::Wait;
            return (clone, PlayerEvent::Wait, next_pos);
        }

        if next_pos.0 >= self.width as i32 ||
           next_pos.0 < 0 ||
           next_pos.1 >= self.height as i32 ||
           next_pos.1 < 0 {
            clone = self.fallover(player);
            return (clone, PlayerEvent::FallOver, clone.position)
        }


        clone.recent_event = p_event;

        (clone, p_event, next_pos)
    }

    fn try_traverse(&self, player: &Player, next_pos: (i32, i32, i32), up_speed_fact: f32, down_speed_fact: f32) -> (Player, PlayerEvent) {
        // check if next_pos is adjacent to current position
        let mut clone = Player::clone(player);
        let mut p_event = PlayerEvent::Wait;

        if self.can_traverse(player.xy(), 
                            (next_pos.0, next_pos.1)) {
            // change speed when height changes
            if next_pos.2 < player.position.2 {
                clone.speed.0 *= down_speed_fact;
                clone.speed.1 *= down_speed_fact;
            }

            else if next_pos.2 > player.position.2 {
                clone.speed.0 *= up_speed_fact;
                clone.speed.1 *= up_speed_fact;
            }

            if clone.x() != next_pos.0 || clone.y() != next_pos.1 {
                p_event = PlayerEvent::Move;

                // move player to next position
                clone.position = next_pos;
            } else {
                if let Obstacle::Rail(_, _) = self.get_obstacle(clone.x(), clone.y()) {
                    clone = self.fallover(player);
                    p_event = PlayerEvent::FallOver;
                }
            }
        // fallover if we cannot traverse to next_pos
        // and do not update the player's position
        } else {
            clone = self.fallover(player);
            p_event = PlayerEvent::FallOver;
        }
        
        /*
        let mut to_remove = Vec::new();
        for index in (0..self.get_goals().len()).rev() {
            if self.goals[index].0 == clone.x() && self.goals[index].1 == clone.y() {
                to_remove.push(index);
            }
        }
        for index in to_remove {
            self.goals.remove(index);
        }
        */

        (clone, p_event)
    }
}
