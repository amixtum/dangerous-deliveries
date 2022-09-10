use console_engine::KeyCode;

use std::collections::{hash_map, HashMap};

use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::Player;

pub struct LookMode {
    key_map: HashMap<KeyCode, (i32, i32)>,
}

impl LookMode {
    pub fn new() -> Self {
        let mut lm = LookMode {
            key_map: HashMap::new(),
        };

        // left
        lm.key_map.insert(KeyCode::Char('h'), (-1, 0));
        lm.key_map.insert(KeyCode::Char('a'), (-1, 0));

        // right
        lm.key_map.insert(KeyCode::Char('l'), (1, 0));
        lm.key_map.insert(KeyCode::Char('d'), (1, 0));

        // up
        lm.key_map.insert(KeyCode::Char('k'), (0, -1));
        lm.key_map.insert(KeyCode::Char('w'), (0, -1));

        // down
        lm.key_map.insert(KeyCode::Char('j'), (0, 1));
        lm.key_map.insert(KeyCode::Char('s'), (0, 1));

        // up right
        lm.key_map.insert(KeyCode::Char('u'), (1, -1));
        lm.key_map.insert(KeyCode::Char('e'), (1, -1));

        // up left
        lm.key_map.insert(KeyCode::Char('y'), (-1, -1));
        lm.key_map.insert(KeyCode::Char('q'), (-1, -1));

        // down left
        lm.key_map.insert(KeyCode::Char('b'), (-1, 1));
        lm.key_map.insert(KeyCode::Char('z'), (-1, 1));

        // down right
        lm.key_map.insert(KeyCode::Char('n'), (1, 1));
        lm.key_map.insert(KeyCode::Char('c'), (1, 1));

        // here
        lm.key_map.insert(KeyCode::Char('.'), (0, 0));
        lm.key_map.insert(KeyCode::Tab, (0, 0));

        lm
    }
}

impl LookMode {
    pub fn get_keys(&self) -> hash_map::Keys<KeyCode, (i32, i32)> {
        self.key_map.keys()
    }

    pub fn get_direction(&self, key: KeyCode) -> Option<&(i32, i32)> {
        self.key_map.get(&key)
    }

    pub fn describe_direction(
        &self,
        table: &ObstacleTable,
        player: &Player,
        key: KeyCode,
    ) -> String {
        let mut s = String::new();

        if let Some(direction) = self.get_direction(key) {
            let x = player.x() + direction.0;
            let y = player.y() + direction.1;
            match table.get_obstacle(x, y) {
                Obstacle::Platform(_) => {
                    s.push_str("Platform ");
                }
                Obstacle::Pit => {
                    s.push_str("Bottomless Pit ");
                }
                Obstacle::Rail(_, dir) => {
                    let x_dir = dir.0.round() as i32;
                    let y_dir = dir.1.round() as i32;

                    if x_dir == 0 && y_dir == -1 {
                        s.push_str("Up Rail");
                    } else if x_dir == 0 && y_dir == 1 {
                        s.push_str("Down Rail");
                    } else if x_dir == 1 && y_dir == -1 {
                        s.push_str("UpRight Rail");
                    } else if x_dir == 1 && y_dir == 0 {
                        s.push_str("Right Rail");
                    } else if x_dir == 1 && y_dir == 1 {
                        s.push_str("DownRight Rail");
                    } else if x_dir == -1 && y_dir == -1 {
                        s.push_str("UpLeft Rail");
                    } else if x_dir == -1 && y_dir == 0 {
                        s.push_str("Left Rail");
                    } else if x_dir == -1 && y_dir == 1 {
                        s.push_str("DownLeft Rail");
                    }
                }
            }
        }

        s
    }
}
