use rltk::VirtualKeyCode;

use std::collections::{hash_map, HashMap};

use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::Player;

pub struct LookMode {
    key_map: HashMap<VirtualKeyCode, (i32, i32)>,
}

impl LookMode {
    pub fn new() -> Self {
        let mut lm = LookMode {
            key_map: HashMap::new(),
        };

        // left
        lm.key_map.insert(VirtualKeyCode::H, (-1, 0));
        lm.key_map.insert(VirtualKeyCode::A, (-1, 0));

        // right
        lm.key_map.insert(VirtualKeyCode::L, (1, 0));
        lm.key_map.insert(VirtualKeyCode::D, (1, 0));

        // up
        lm.key_map.insert(VirtualKeyCode::K, (0, -1));
        lm.key_map.insert(VirtualKeyCode::W, (0, -1));

        // down
        lm.key_map.insert(VirtualKeyCode::J, (0, 1));
        lm.key_map.insert(VirtualKeyCode::S, (0, 1));

        // up right
        lm.key_map.insert(VirtualKeyCode::U, (1, -1));
        lm.key_map.insert(VirtualKeyCode::E, (1, -1));

        // up left
        lm.key_map.insert(VirtualKeyCode::Y, (-1, -1));
        lm.key_map.insert(VirtualKeyCode::Q, (-1, -1));

        // down left
        lm.key_map.insert(VirtualKeyCode::B, (-1, 1));
        lm.key_map.insert(VirtualKeyCode::Z, (-1, 1));

        // down right
        lm.key_map.insert(VirtualKeyCode::N, (1, 1));
        lm.key_map.insert(VirtualKeyCode::C, (1, 1));

        // wait
        lm.key_map.insert(VirtualKeyCode::Period, (0, 0));
        lm.key_map.insert(VirtualKeyCode::Tab, (0, 0));

        lm
    }
}

impl LookMode {
    pub fn get_keys(&self) -> hash_map::Keys<VirtualKeyCode, (i32, i32)> {
        self.key_map.keys()
    }

    pub fn get_direction(&self, key: VirtualKeyCode) -> Option<&(i32, i32)> {
        self.key_map.get(&key)
    }

    pub fn describe_direction(
        &self,
        table: &ObstacleTable,
        player: &Player,
        key: VirtualKeyCode,
    ) -> String {
        let mut s = String::new();

        if let Some(direction) = self.get_direction(key) {
            let x = player.x() + direction.0;
            let y = player.y() + direction.1;
            match table.get_obstacle(x, y) {
                Obstacle::Wall => {
                    s.push_str("Wall ");
                }
                Obstacle::Platform => {
                    s.push_str("Platform ");
                }
                Obstacle::Pit => {
                    s.push_str("Bottomless Pit ");
                }
                Obstacle::Rail(x_dir, y_dir) => {
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
