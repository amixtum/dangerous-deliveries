use std::collections::HashMap;

use util::vec_ops;

use super::obstacle::Obstacle;
use super::obstacle_table::ObstacleTable;

use super::direction::Direction;

pub fn apply_automata(table: &mut ObstacleTable) {
    let mut next = HashMap::new();
    for x in 0..table.width() {
        for y in 0..table.height() {
            next.insert(
                (x as i32, y as i32),
                compute_next(&table, x as i32, y as i32),
            );
        }
    }

    for x in 0..table.width() {
        for y in 0..table.height() {
            if let Some(obstacle) = next.remove(&(x as i32, y as i32)) {
                table.set_obstacle((x as i32, y as i32), obstacle);
            }
        }
    }

    table.update_platforms();
    table.compute_unions();
}

pub fn compute_next(obs_table: &ObstacleTable, x: i32, y: i32) -> Obstacle {
    let neighbors = vec_ops::neighbors_set(
        (x, y),
        (0, 0),
        (obs_table.width() as i32 - 1, obs_table.height() as i32 - 1),
    );
    let mut count_rail = 0;
    let mut count_platform = 0;
    let mut count_wall = 0;
    for neighbor in neighbors.iter() {
        match obs_table.get_obstacle(neighbor.0, neighbor.1) {
            Obstacle::Pit => {}
            Obstacle::Wall => count_wall += 1,
            Obstacle::Platform => {
                count_platform += 1;
            }
            Obstacle::Rail(_, _) => {
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

    match obs_table.get_obstacle(x, y) {
        Obstacle::Pit => {
            return Obstacle::Pit;
        }
        Obstacle::Wall => {
            if count_wall > 3 {
                return Obstacle::Platform;
            } else if count_platform > 2 {
                return Obstacle::Wall;
            } else {
                /*
                let directions = dir_map.keys().collect::<Vec<_>>();
                let mut rng = rand::thread_rng();
                if let Some((xdir, ydir)) = directions.choose(&mut rng) {
                    return Obstacle::Rail(*xdir, *ydir);
                }
                */

                return Obstacle::Platform;
            }
        }
        Obstacle::Platform => {
            if count_platform > 3 {
                /*
                let dirs: Vec<(Direction, (i32, i32))> = neighbors
                    .iter()
                    .map(|p| match obs_table.get_obstacle(p.0, p.1) {
                        Obstacle::Rail(xdir, ydir) => {
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
                    let fchoice = (choice.1 .0, choice.1 .1);
                    match choice.0 {
                        Direction::NorthEast => {
                            return Obstacle::Wall;
                        }
                        Direction::NorthWest => {
                            return Obstacle::Rail(fchoice.0, fchoice.1);
                        }
                        Direction::SouthEast => {
                            return Obstacle::Rail(fchoice.0, fchoice.1);
                        }
                        Direction::SouthWest => {
                            return Obstacle::Wall;
                        }
                        _ => {}
                    }
                }
                */
                return Obstacle::Wall;
            } else if count_wall <= 2 {
                return Obstacle::Wall;
            } else {
                return Obstacle::Wall;
            }
        }
        Obstacle::Rail(_, _) => {
            if count_rail == 3 {
                return Obstacle::Platform;
            } else if count_rail < 2 {
                return Obstacle::Wall;
                //let dir = (xdir, ydir);
                //return Obstacle::Rail(obs_table.get_height(x, y), (dir.0 as f32, dir.1 as f32));
            } else {
                return Obstacle::Platform;
            }
        }
    }
}
