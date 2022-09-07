use rand::Rng;
use rand::prelude::SliceRandom;

use util::vec_ops;

use super::obstacle::{Obstacle, ObstacleType};
use super::obstacle_table::ObstacleTable;

pub fn compute_next(obs_table: &ObstacleTable, x: i32, y: i32) -> Obstacle {
    let neighbors = vec_ops::neighbors((x, y), (0, 0), (obs_table.width() as i32 - 1, obs_table.height() as i32 - 1));
    let mut count_rail = 0;
    let mut count_platform = 0;
    let mut index = 0;
    while index < neighbors.len() {
        match obs_table.get_obstacle_type(neighbors[index].0, neighbors[index].1) {
            ObstacleType::Pit => { },
            ObstacleType::Platform => {
                count_platform += 1;
            },
            ObstacleType::Rail(_, _) => {
                count_rail += 1;
            },
        }

        index += 1;
    }

    match obs_table.get_obstacle_type(x, y) {
        ObstacleType::Pit => {
            return Obstacle::Pit;
        },
        ObstacleType::Platform => {
            if count_platform > count_rail && count_platform > 1 {
                return Obstacle::Platform(obs_table.get_height(x, y));
            } 
            else {
                let mut rng = rand::thread_rng();
                let dirs: Vec<(i32, i32)>= neighbors.iter().map(|p| {
                    match obs_table.get_obstacle_type(p.0, p.1) {
                        ObstacleType::Rail(dx, dy) => (dx, dy),
                        _ => (0, 0),
                    }
                }).filter(|p| {
                    if p.0 == 0 && p.1 == 0 {
                        return false;
                    }
                    true
                }).collect();
                if let Some(choice) = dirs.choose(&mut rng) {
                    return Obstacle::Rail(obs_table.get_height(x, y), (choice.0 as f32, choice.1 as f32));
                }
                return Obstacle::Platform(obs_table.get_height(x, y));
            }
        },
        ObstacleType::Rail(xdir, ydir) => {
            if count_rail > count_platform {
                if rand::thread_rng().gen_bool(0.1) {
                    let mut rng = rand::thread_rng();
                    let dirs: Vec<(i32, i32)>= neighbors.iter().map(|p| {
                        match obs_table.get_obstacle_type(p.0, p.1) {
                            ObstacleType::Rail(dx, dy) => (dx, dy),
                            _ => (0, 0),
                        }
                    }).filter(|p| {
                        if p.0 == 0 && p.1 == 0 {
                            return false;
                        }
                        true
                    }).collect();
                    if let Some(choice) = dirs.choose(&mut rng) {
                        return Obstacle::Rail(obs_table.get_height(x, y), (choice.0 as f32, choice.1 as f32));
                    }
                    return Obstacle::Platform(obs_table.get_height(x, y));
                }
                return Obstacle::Platform(obs_table.get_height(x, y));
            }
            else {
                let mut dir = (xdir, ydir);
                if rand::thread_rng().gen_bool(0.5) {
                    dir = vec_ops::rotate_left((xdir, ydir));
                }
                return Obstacle::Rail(obs_table.get_height(x, y), (dir.0 as f32, dir.1 as f32));
            }
        },
    }
}
