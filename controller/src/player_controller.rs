use console_engine::KeyCode;

use std::collections::HashMap;
use std::collections::hash_map;
use std::fs;

use model::player::Player;
use model::player_event::PlayerEvent;
use model::cell_table::CellTable;

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
    pub fn get_keys(&self) -> hash_map::Keys<KeyCode, (f32, f32)> {
        self.key_map.keys()
    }

    pub fn get_inst_velocity(&self, key: KeyCode) -> Option<&(f32, f32)> {
        self.key_map.get(&key)
    }

    pub fn move_player(&self, table: &CellTable, player: &Player, key: KeyCode) -> (Player, PlayerEvent) {
        if let Some(inst_v) = self.get_inst_velocity(key) {
            return table.compute_move(
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
        return table.compute_move(
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
}
