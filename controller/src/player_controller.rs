use console_engine::KeyCode;

use std::collections::HashMap;
use std::collections::hash_map;
use std::fs;

use model::player::Player;
use model::player_event::PlayerEvent;
use model::obstacle_table::ObstacleTable;
use model::obstacle::Obstacle;

use util::vec_ops;

pub struct PlayerController {
    key_map: HashMap<KeyCode, (f32, f32)>,
    pub speed_damp: f32, 
    pub balance_damp: f32, 
    pub turn_factor: f32,
    pub onrail_balance_factor: f32,
    pub offrail_balance_factor: f32,
    pub up_speed_factor: f32,
    pub down_speed_factor: f32,
    pub max_speed: f32,
    pub fallover_threshold: f32,
}

impl PlayerController {
    pub fn new(conf_file: &str) -> Self {
        let mut speed_damp: f32 = 0.5;
        let mut balance_damp: f32 = 0.75;
        let mut turn_factor: f32 = 0.25;
        let mut onrail_balance_factor: f32 = 0.25;
        let mut offrail_balance_factor: f32 = 0.25;
        let mut up_speed_factor: f32 = 0.5;
        let mut down_speed_factor: f32 = 1.5;
        let mut max_speed: f32 = 3.0;
        let mut fallover_threshold: f32 = 5.0;

        // read conf file
        if let Ok(contents) = fs::read_to_string(conf_file) {
            for line in contents.lines() {
                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                if words.len() == 2 {
                    if words[0] == "speed_damp" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            speed_damp = num; 
                        }
                    }
                    else if words[0] == "balance_damp" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            balance_damp = num; 
                        }
                    }
                    else if words[0] == "turn_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            turn_factor = num; 
                        }
                    }
                    else if words[0] == "offrail_balance_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            offrail_balance_factor = num; 
                        }
                    }
                    if words[0] == "turn_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            turn_factor = num; 
                        }
                    }
                    else if words[0] == "onrail_balance_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            onrail_balance_factor = num; 
                        }
                    }
                    else if words[0] == "up_speed_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            up_speed_factor = num; 
                        }
                    }
                    else if words[0] == "down_speed_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            down_speed_factor = num; 
                        }
                    }
                    else if words[0] == "max_speed" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            max_speed = num; 
                        }
                    }
                    else if words[0] == "fallover_threshold" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            fallover_threshold = num; 
                        }
                    }
                }
            }
        }

        let mut pc = PlayerController {
            key_map: HashMap::new(),
            speed_damp,
            balance_damp,
            turn_factor,
            onrail_balance_factor,
            offrail_balance_factor,
            up_speed_factor,
            down_speed_factor,
            max_speed,
            fallover_threshold,
        };

        // left
        pc.key_map.insert(KeyCode::Char('h'), (-1.0, 0.0));
        pc.key_map.insert(KeyCode::Char('a'), (-1.0, 0.0));

        // right
        pc.key_map.insert(KeyCode::Char('l'), (1.0, 0.0));
        pc.key_map.insert(KeyCode::Char('d'), (1.0, 0.0));

        // up
        pc.key_map.insert(KeyCode::Char('k'), (0.0, -1.0));
        pc.key_map.insert(KeyCode::Char('w'), (0.0, -1.0));

        // down
        pc.key_map.insert(KeyCode::Char('j'), (0.0, 1.0));
        pc.key_map.insert(KeyCode::Char('s'), (0.0, 1.0));

        // up right
        pc.key_map.insert(KeyCode::Char('u'), (1.0, -1.0));
        pc.key_map.insert(KeyCode::Char('e'), (1.0, -1.0));

        // up left
        pc.key_map.insert(KeyCode::Char('y'), (-1.0, -1.0));
        pc.key_map.insert(KeyCode::Char('q'), (-1.0, -1.0));

        // down left
        pc.key_map.insert(KeyCode::Char('b'), (-1.0, 1.0));
        pc.key_map.insert(KeyCode::Char('z'), (-1.0, 1.0));

        // down right
        pc.key_map.insert(KeyCode::Char('n'), (1.0, 1.0));
        pc.key_map.insert(KeyCode::Char('c'), (1.0, 1.0));

        // wait
        pc.key_map.insert(KeyCode::Char('.'), (0.0, 0.0));
        pc.key_map.insert(KeyCode::Tab, (0.0, 0.0));

        pc
    }
}

impl PlayerController {
     pub fn set_model(&mut self,     
                      speed_damp: f32, 
                      balance_damp: f32, 
                      turn_factor: f32,
                      onrail_balance_factor: f32,
                      offrail_balance_factor: f32,
                      up_speed_factor: f32,
                      down_speed_factor: f32,
                      max_speed: f32,
                      fallover_threshold: f32,) {
        self.speed_damp = speed_damp;
        self.balance_damp = balance_damp;
        self.turn_factor = turn_factor;
        self.onrail_balance_factor = onrail_balance_factor;
        self.offrail_balance_factor = offrail_balance_factor;
        self.up_speed_factor = up_speed_factor;
        self.down_speed_factor = down_speed_factor;
        self.max_speed = max_speed;
        self.fallover_threshold = fallover_threshold;
     }

    pub fn get_keys(&self) -> hash_map::Keys<KeyCode, (f32, f32)> {
        self.key_map.keys()
    }

    pub fn get_inst_velocity(&self, key: KeyCode) -> Option<&(f32, f32)> {
        self.key_map.get(&key)
    }

    pub fn move_player(&self, table: &ObstacleTable, player: &Player, key: KeyCode) -> Player {
        if let Some(inst_v) = self.get_inst_velocity(key) {
            return PlayerController::compute_move(
                table,
                player,
                *inst_v,
                self.speed_damp,
                self.balance_damp,
                self.turn_factor,
                self.onrail_balance_factor,
                self.up_speed_factor,
                self.down_speed_factor,
                self.max_speed,
                self.fallover_threshold,
            );
        }
        return PlayerController::compute_move(
            table,
            player,
            (0.0, 0.0),
            self.speed_damp,
            self.balance_damp,
            self.turn_factor,
            self.onrail_balance_factor,
            self.up_speed_factor,
            self.down_speed_factor,
            self.max_speed,
            self.fallover_threshold,
        );
    }

    pub fn reset_player(table: &ObstacleTable, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.position = (table.width() as i32 / 2, table.height() as i32 / 2, table.get_height(table.width() as i32 / 2, table.height() as i32 / 2));
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.recent_event = PlayerEvent::GameOver(clone.time as i32);
        clone.time = 0.0;
        clone.n_falls = 0;
        clone
    }

    pub fn compute_move(table: &ObstacleTable, 
                        player: &Player,
                        (inst_x, inst_y): (f32, f32),
                        speed_damp: f32, 
                        balance_damp: f32, 
                        turn_fact: f32,
                        onrail_balance_fact: f32,
                        up_speed_fact: f32,
                        down_speed_fact: f32,
                        max_speed: f32,
                        fallover_threshold: f32,) -> Player {
        // compute initial speed and balance values
        let mut player = PlayerController::compute_initial_speed_balance(
            table,
            player, 
            (inst_x, inst_y), 
            max_speed, 
            speed_damp, 
            balance_damp, 
            turn_fact,
        );

        // compute position and updated player fields
        let result = PlayerController::compute_next_position(
            table, 
            &player, 
            (inst_x, inst_y), 
            onrail_balance_fact
        );

        player = result.0;
        let next_pos = result.1;

        player.time += 1.0 / (1.0 + vec_ops::magnitude(player.speed));

        // fallover if the player is off balance
        if player.balance.0.abs() >= fallover_threshold || player.balance.1.abs() >= fallover_threshold {
            return PlayerController::fallover(table, &player);
        }

        // fall into a pit. Game Over
        if let Obstacle::Pit = table.get_obstacle(next_pos.0, next_pos.1) {
            // reset the player
            return PlayerController::reset_player(table, &player);
        }

        // try to move player to next_pos
        // updating speed values depending on the change
        // in height after the move
        // and return the updated Player
        return PlayerController::try_traverse(table, &player, next_pos, up_speed_fact, down_speed_fact);
    }

    fn fallover(table: &ObstacleTable, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.recent_event = PlayerEvent::FallOver;
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.n_falls += 1;

        match table.get_obstacle(player.x(), player.y()) {
            Obstacle::Rail(height, _) => {
                let neighbors = vec_ops::neighbors(player.xy(), 
                                                   (0, 0), 
                                                   (table.width() as i32 - 1, table.height() as i32 - 1));
                let mut found = false;

                for neighbor in neighbors {
                    match table.get_obstacle(neighbor.0, neighbor.1) {
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
                    clone = PlayerController::reset_player(table, player);
                }
            },
            _ => { } 
        }

        clone
    }

    fn compute_initial_speed_balance(table: &ObstacleTable, 
                                     player: &Player, 
                                     (inst_x, inst_y): (f32, f32), 
                                     max_speed: f32, 
                                     speed_damp: f32, 
                                     balance_damp: f32, 
                                     turn_fact: f32,) -> Player {
        let last_speed = player.speed;
        let last_obstacle = table.get_obstacle(player.position.0, 
                                               player.position.1);

        let mut clone = Player::clone(player);

        match last_obstacle {
            Obstacle::Platform(_) => {
                // compute speed
                clone.speed.0 = clone.speed.0 * speed_damp + inst_x;
                clone.speed.1 = clone.speed.1 * speed_damp + inst_y;

                clone.speed.0 = clone.speed.0.clamp(-max_speed, max_speed);
                clone.speed.1 = clone.speed.1.clamp(-max_speed, max_speed);

                // compute balance
                clone.balance.0 = clone.balance.0 * balance_damp + 
                                        (inst_y * clone.speed.0) * turn_fact;

                clone.balance.1 = clone.balance.1 * balance_damp + 
                                        (inst_x * clone.speed.1) * turn_fact;
            },
            Obstacle::Rail(_, (x_dir, y_dir)) => {
                // compute speed
                clone.speed.0 = clone.speed.0 * speed_damp + x_dir + inst_x;
                clone.speed.1 = clone.speed.1 * speed_damp + y_dir + inst_y;

                clone.speed.0 = clone.speed.0.clamp(-max_speed, max_speed);
                clone.speed.1 = clone.speed.1.clamp(-max_speed, max_speed);

                // compute balance
                clone.balance.0 = clone.balance.0 * balance_damp + 
                                        (inst_y * clone.speed.0) * turn_fact;

                clone.balance.1 = clone.balance.1 * balance_damp + 
                                        (inst_x * clone.speed.1) * turn_fact;
            }
            _ => { }
        }

        clone
    }

    // updated a player's balance so it must return a new player as well
    fn compute_onrail(table: &ObstacleTable, player: &Player, (inst_x, inst_y): (f32, f32), (x_dir, y_dir): (f32, f32), onrail_balance_fact: f32) -> (Player, (i32, i32, i32)) {
        let (unit_x, unit_y) = vec_ops::discrete_jmp((inst_x, inst_y));
        let mut next_pos = player.position;
        next_pos.0 = (player.position.0 + unit_x).clamp(0, table.width() as i32 - 1);
        next_pos.1 = (player.position.1 + unit_y).clamp(0, table.height() as i32 - 1);
        next_pos.2 = table.get_height(next_pos.0, next_pos.1);

        let mut clone = Player::clone(player);

        clone.balance.0 += (clone.speed.0 - x_dir * clone.speed.0) * onrail_balance_fact;
        clone.balance.1 += (clone.speed.1 - y_dir * clone.speed.1) * onrail_balance_fact;

        (clone, next_pos)
    }

    fn compute_continue(table: &ObstacleTable, player: &Player) -> (i32, i32, i32) {
        let mut next_pos = player.position;
        let temp = player.speed.0;
        next_pos.0 = (next_pos.0 + temp as i32).clamp(next_pos.0 - 1, next_pos.0 + 1);
        next_pos.0 = next_pos.0.clamp(0, table.width() as i32 - 1);

        let temp = player.speed.1;
        next_pos.1 = (next_pos.1 + temp as i32).clamp(next_pos.1 - 1, next_pos.1 + 1);
        next_pos.1 = next_pos.1.clamp(0, table.height() as i32 - 1);

        next_pos.2 = table.get_height(next_pos.0, next_pos.1);

        next_pos
    }

    // Note: Must call compute_intitial_speed_balance first in order to update speed and balance
    // values, otherwise, this will compute the next position without taking into account user
    // input
    fn compute_next_position(table: &ObstacleTable, player: &Player, (inst_x, inst_y): (f32, f32), onrail_balance_fact: f32) -> (Player, (i32, i32, i32)) {
        let mut next_pos = player.position;
        let last_obstacle = table.get_obstacle(player.x(), player.y());
        let units = vec_ops::discrete_jmp((inst_x, inst_y));
        let unit_x = units.0;
        let unit_y = units.1;

        let mut clone = Player::clone(player);
        clone.recent_event = PlayerEvent::Move;

        // bump into border
        if player.x() + unit_x >= table.width() as i32 ||
           player.x() + unit_x < 0 ||
           player.y() + unit_y >= table.height() as i32 ||
           player.y() + unit_y < 0 {
            clone = PlayerController::fallover(table, player);
            return (clone, clone.position)
        }

        // compute position
        let obs_at_next = table.get_obstacle(player.x() + unit_x, player.y() + unit_y);
        match last_obstacle {
            Obstacle::Rail(last_height, _) => {
                match obs_at_next {
                    Obstacle::Rail(height, _)=> {
                        if (height - last_height).abs() > 1 {
                            clone = PlayerController::fallover(table, player);
                        }
                        else {
                            next_pos = PlayerController::compute_continue(table, player);
                            clone.recent_event = PlayerEvent::OffRail;
                        }
                    },
                    _ => {
                        next_pos = PlayerController::compute_continue(table, player);
                        clone.recent_event = PlayerEvent::OnRail;
                    },
                }
            },
            Obstacle::Platform(_) => {
                match obs_at_next {
                    Obstacle::Rail(_, (x_dir, y_dir)) => {
                        let result = PlayerController::compute_onrail(table, player, (inst_x, inst_y), (x_dir, y_dir), onrail_balance_fact);
                        clone = result.0;
                        next_pos = result.1;
                        clone.recent_event = PlayerEvent::OnRail;
                    },
                    _ => {
                        next_pos = PlayerController::compute_continue(table, player);
                    },
                }
            },

            _ => {},
        }

        if next_pos.0 == player.x() && next_pos.1 == player.y() {
            clone.recent_event = PlayerEvent::Wait;
            return (clone, next_pos);
        }

        if next_pos.0 >= table.width() as i32 ||
           next_pos.0 < 0 ||
           next_pos.1 >= table.height() as i32 ||
           next_pos.1 < 0 {
            clone = PlayerController::fallover(table, &clone);
            return (clone, clone.position)
        }

        (clone, next_pos)
    }

    fn try_traverse(table: &ObstacleTable, player: &Player, next_pos: (i32, i32, i32), up_speed_fact: f32, down_speed_fact: f32) -> Player {
        // check if next_pos is adjacent to current position
        let mut clone = Player::clone(player);

        if table.can_traverse(player.xy(), 
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
                // move player to next position
                clone.position.0 = next_pos.0.clamp(0, table.width() as i32 - 1);
                clone.position.1 = next_pos.1.clamp(0, table.height() as i32 - 1);
                clone.position.2 = table.get_height(clone.x(), clone.y());
            } 

        // fallover if we cannot traverse to next_pos
        // and do not update the player's position
        } else {
            clone = PlayerController::fallover(table, player);
        }

        clone
    }
}
