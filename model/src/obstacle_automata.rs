use rand::Rng;

use util::vec_ops;

use super::obstacle::{Obstacle, ObstacleType};
use super::obstacle_table::ObstacleTable;

pub fn compute_next(obs_table: &ObstacleTable, x: i32, y: i32) -> Obstacle {
    let neighbors = vec_ops::neighbors((x, y), (0, 0), (obs_table.width() as i32 - 1, obs_table.height() as i32 - 1));
    let mut count_rail = 0;
    let mut count_platform = 0;
    for neighbor in neighbors {
        match obs_table.get_obstacle_type(neighbor.0, neighbor.1) {
            ObstacleType::Pit => { },
            ObstacleType::Platform => {
                count_platform += 1;
            },
            ObstacleType::Rail(_, _) => {
                count_rail += 1;
            },
        }
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
                let mut dir = (1, 1);
                for _ in 0..(rand::thread_rng().gen_range(1..7) as i32) {
                    dir = vec_ops::rotate_left(dir);
                }
                return Obstacle::Rail(obs_table.get_height(x, y), (dir.0 as f32, dir.1 as f32));
            }
        },
        ObstacleType::Rail(xdir, ydir) => {
            if count_rail > count_platform {
                return Obstacle::Platform(obs_table.get_height(x, y));
            }
            else {
                let dir = vec_ops::rotate_left((xdir, ydir));
                return Obstacle::Rail(obs_table.get_height(x, y), (dir.0 as f32, dir.1 as f32));
            }
        },
    }
}
