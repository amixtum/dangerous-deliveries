use std::f32::consts::PI;
//use std::collections::HashSet;
use std::collections::HashMap;

//use rand::Rng;
use rand::prelude::SliceRandom;

use util::vec_ops;

use super::obstacle::{Obstacle, ObstacleType};
use super::obstacle_table::ObstacleTable;

use super::direction::Direction;

pub fn compute_next(obs_table: &ObstacleTable, x: i32, y: i32) -> Obstacle {
    let neighbors = vec_ops::neighbors_set(
        (x, y),
        (0, 0),
        (obs_table.width() as i32 - 1, obs_table.height() as i32 - 1),
    );
    let mut count_rail = 0;
    let mut count_platform = 0;
    for neighbor in neighbors.iter() {
        match obs_table.get_obstacle_type(neighbor.0, neighbor.1) {
            ObstacleType::Pit => {}
            ObstacleType::Platform => {
                count_platform += 1;
            }
            ObstacleType::Rail(_, _) => {
                count_rail += 1;
            }
        }
    }

    let mut dir_map = HashMap::new();
    dir_map.insert((0, -1), Direction::Up);
    dir_map.insert((0, 1), Direction::Down);
    dir_map.insert((-1, 0), Direction::Left);
    dir_map.insert((1, 0), Direction::Right);
    dir_map.insert((-1, -1), Direction::NorthWest);
    dir_map.insert((1, -1), Direction::NorthEast);
    dir_map.insert((-1, 1), Direction::SouthWest);
    dir_map.insert((1, 1), Direction::SouthEast);

    match obs_table.get_obstacle_type(x, y) {
        ObstacleType::Pit => {
            return Obstacle::Pit;
        }
        ObstacleType::Platform => {
            if count_rail > count_platform + 1 {
                return Obstacle::Platform(obs_table.get_height(x, y));
            } else {
                let dirs: Vec<(Direction, (i32, i32))> = neighbors
                    .iter()
                    .map(|p| match obs_table.get_obstacle_type(p.0, p.1) {
                        ObstacleType::Rail(xdir, ydir) => {
                            let dir = (p.0 - x, p.1 - y);
                            if let Some(dir) = dir_map.get(&(dir.0, dir.1)) {
                                return (*dir, (xdir, ydir));
                            }
                            return (Direction::Center, (0, 0));
                        }
                        _ => (Direction::Center, (0, 0)),
                    })
                    .filter(|d| match d.0 {
                        Direction::Center => false,
                        _ => true,
                    })
                    .collect();
                let mut rng = rand::thread_rng();
                if let Some(choice) = dirs.choose(&mut rng) {
                    let fchoice = (choice.1 .0 as f32, choice.1 .1 as f32);
                    match choice.0 {
                        Direction::NorthEast => {
                            return Obstacle::Rail(obs_table.get_height(x, y), fchoice);
                        }
                        Direction::NorthWest => {
                            return Obstacle::Rail(obs_table.get_height(x, y), fchoice);
                        }
                        Direction::SouthEast => {
                            return Obstacle::Rail(obs_table.get_height(x, y), fchoice);
                        }
                        Direction::SouthWest => {
                            return Obstacle::Rail(obs_table.get_height(x, y), fchoice);
                        }
                        _ => {}
                    }
                }
                return Obstacle::Platform(obs_table.get_height(x, y));
            }
        }
        ObstacleType::Rail(xdir, ydir) => {
            if count_rail == 3 {
                let dirs: Vec<Direction> = neighbors
                    .iter()
                    .map(|p| match obs_table.get_obstacle_type(p.0, p.1) {
                        ObstacleType::Rail(_, _) => {
                            let dir = (p.0 - x, p.1 - y);
                            if let Some(dir) = dir_map.get(&(dir.0, dir.1)) {
                                return *dir;
                            }
                            return Direction::Center;
                        }
                        _ => Direction::Center,
                    })
                    .filter(|d| match d {
                        Direction::Center => false,
                        _ => true,
                    })
                    .collect();
                let mut rng = rand::thread_rng();
                if let Some(choice) = dirs.choose(&mut rng) {
                    match *choice {
                        Direction::NorthEast
                        | Direction::NorthWest
                        | Direction::SouthEast
                        | Direction::SouthWest => {
                            let dir = vec_ops::rotate((xdir, ydir), PI / 4.0);
                            return Obstacle::Rail(
                                obs_table.get_height(x, y),
                                (dir.0 as f32, dir.1 as f32),
                            );
                        }
                        _ => {}
                    }
                    return Obstacle::Platform(obs_table.get_height(x, y));
                }
                return Obstacle::Platform(obs_table.get_height(x, y));
            } else if count_rail < 3 || count_rail == 5 {
                let dir = (xdir, ydir);
                return Obstacle::Rail(obs_table.get_height(x, y), (dir.0 as f32, dir.1 as f32));
            } else {
                return Obstacle::Platform(obs_table.get_height(x, y));
            }
        }
    }
}
