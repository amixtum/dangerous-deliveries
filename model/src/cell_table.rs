use super::obstacle::Obstacle;
use super::player::Player;
use super::cell::Cell;
use super::player_event::PlayerEvent;
use super::util;


pub struct CellTable {
    pub width: usize,
    pub height: usize,
    pub table: Vec<Vec<Cell>>,
}

impl CellTable {
    pub fn new(width: usize, height: usize) -> Self {
        let mut ct = CellTable {
            width,
            height,
            table: Vec::new(),
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


impl CellTable {
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

    pub fn reset_player(&self, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.position = (0, 0, self.get_height(0, 0));
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
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

        let mut p_event = PlayerEvent::Move;

        // compute initial speed and balance values
        let mut player = self.compute_initial_speed_balance(player, (inst_x, inst_y), max_speed, speed_damp, balance_damp, turn_fact);

        // compute position and updated player fields
        let result = self.compute_next_position(&player, (inst_x, inst_y), onrail_balance_fact, offrail_balance_fact);
        player = result.0;
        let next_pos = result.1;

        // fallover if the player is off balance
        if util::magnitude(player.balance) >= fallover_threshold {
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
        self.try_traverse(&player, next_pos, up_speed_fact, down_speed_fact)
    }

    fn fallover(&self, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);

        match self.get_obstacle(player.x(), player.y()) {
            Obstacle::Rail(height, _) => {
                let neighbors = util::neighbors(player.xy(), 
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
                clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;
                clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;

                // slow down some more if the passed instantaneous velocity
                // is in the opposite direction of the rail direction
                if inst_x > 0.0 && x_dir < 0.0 && inst_y > 0.0 && y_dir < 0.0 {
                     clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;                    
                     clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;
                } 
                else if inst_x < 0.0 && x_dir > 0.0 && inst_y < 0.0 && y_dir > 0.0 {
                     clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;
                     clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;
                }
                else if  inst_x < 0.0 && x_dir > 0.0 && inst_y > 0.0 && y_dir < 0.0 {
                     clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;
                     clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;                    
                }
                else if  inst_x > 0.0 && x_dir < 0.0 && inst_y > 0.0 && y_dir < 0.0 {
                     clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;
                     clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_x > 0.0 && x_dir < 0.0 && y_dir.abs() < 0.01 {
                    clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_y > 0.0 && y_dir < 0.0 && x_dir.abs() < 0.01 {
                    clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_x < 0.0 && x_dir > 0.0 && y_dir.abs() < 0.01 {
                    clone.speed.0 = x_dir * util::magnitude(clone.speed) * speed_damp;                    
                }
                else if inst_y < 0.0 && y_dir > 0.0 && x_dir.abs() < 0.01 {
                    clone.speed.1 = y_dir * util::magnitude(clone.speed) * speed_damp;                    
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
        let (unit_x, unit_y) = util::discrete_jmp((inst_x, inst_y));
        let mut next_pos = player.position;
        next_pos.0 = player.position.0 + unit_x;
        next_pos.1 = player.position.1 + unit_y;
        next_pos.2 = self.get_height(next_pos.0, next_pos.1);

        let mut clone = Player::clone(player);

        if clone.balance.0 >= clone.balance.1 {
            clone.balance.0 += (clone.speed.0 - (x_dir * util::magnitude(clone.speed))) * onrail_balance_fact;
        } 
        else {
            clone.balance.1 += (clone.speed.1 - (y_dir * util::magnitude(clone.speed))) * onrail_balance_fact;
        }

        (clone, next_pos)
    }

    fn compute_continue(&self, player: &Player) -> (i32, i32, i32) {
        let mut next_pos = player.position;
        let temp = next_pos.0 as f32 + player.speed.0;
        next_pos.0 += temp as i32;

        let temp = next_pos.1 as f32 + player.speed.1;
        next_pos.1 += temp as i32;

        next_pos.2 = self.get_height(next_pos.0, next_pos.1);

        next_pos
    }

    // Note: Must call compute_intitial_speed_balance first in order to update speed and balance
    // values, otherwise, this will compute the next position without taking into account user
    // input
    fn compute_next_position(&self, player: &Player, (inst_x, inst_y): (f32, f32), onrail_balance_fact: f32, offrail_balance_fact: f32) -> (Player, (i32, i32, i32)) {
        let mut next_pos = player.position;
        let last_obstacle = self.get_obstacle(player.x(), player.y());
        let (unit_x, unit_y) = util::discrete_jmp((inst_x, inst_y));
        let obs_at_next = self.get_obstacle(player.x() + unit_x, player.y() + unit_y);

        let mut clone = Player::clone(player);

        // compute position
        match obs_at_next {
            Obstacle::Rail(height, (x_dir, y_dir)) => {
                match last_obstacle {
                    Obstacle::Rail(last_height, (last_x_dir, last_y_dir))=> {
                        if height > last_height {
                            clone = self.fallover(player);
                        } else if last_x_dir != x_dir || last_y_dir != y_dir {
                            let result = self.compute_onrail(player, (inst_x, inst_y), (x_dir, y_dir), onrail_balance_fact);
                            clone = result.0;
                            next_pos = result.1;
                        } else {
                            next_pos = self.compute_continue(player);
                        }
                    },
                    _ => {
                        let result = self.compute_onrail(player, (inst_x, inst_y), (x_dir, y_dir), onrail_balance_fact);
                        clone = result.0;
                        next_pos = result.1;
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
                    },
                    _ => {
                        next_pos = self.compute_continue(player);
                    },
                }
            }
        }

        (clone, next_pos)
    }

    fn try_traverse(&self, player: &Player, next_pos: (i32, i32, i32), up_speed_fact: f32, down_speed_fact: f32) -> (Player, PlayerEvent) {
        // check if next_pos is adjacent to current position
        let mut clone = Player::clone(player);
        let mut p_event = PlayerEvent::Move;

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

            // move player to next position
            clone.position = next_pos;

        // fallover if we cannot traverse to next_pos
        // and do not update the player's position
        } else {
            clone = self.fallover(player);
            p_event = PlayerEvent::FallOver;
        }

        (clone, p_event)
    }
}
