use std::{collections::{HashSet, HashMap}};

use crate::{obstacle_table::ObstacleTable, obstacle::Obstacle};

pub fn get_visible(center: (i32, i32), table: &ObstacleTable, diameter: u32) -> HashSet<(i32, i32)> {
    let mut visible = HashSet::new();

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0),];
    let diags = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut iters = HashMap::new();
    iters.insert((0, -1), HashSet::new());
    iters.insert((0, 1), HashSet::new());
    iters.insert((-1, 0),HashSet::new());
    iters.insert((1, 0), HashSet::new());
    /*
    iters.insert((-1, -1), HashSet::new());
    iters.insert((1, -1), HashSet::new());
    iters.insert((-1, 1), HashSet::new());
    iters.insert((1, 1), HashSet::new());
    */
    for iter in iters.iter_mut() {
        iter.1.insert(center);
    }
    let iters_done = |iters: &HashMap<(i32, i32), HashSet<(i32, i32)>>| {
        for iter in iters.iter() {
            if iter.1.len() < diameter as usize {
                return false;
            }
        }
        return true;
    };

    while !iters_done(&iters) {
        for dir in directions.iter() {
            let set = iters.get(&dir)
                .unwrap()
                .iter()
                .map(|p| { 
                    *p
                 })
                .collect::<Vec<_>>();
            for point in set.iter() {
                let newx = point.0 + dir.0;
                let newy = point.1 + dir.1;
                for dir in directions.iter() {
                    if newx >= 0 && newx < table.width() as i32 &&
                    newy >= 0 && newy < table.height() as i32 {
                        if table.get_obstacle(newx, newy) == Obstacle::Platform  {
                            if iters.get_mut(&dir).unwrap().insert((newx, newy)) {
                                visible.insert((newx, newy));
                            }
                        }
                        else {
                            for dir in diags.iter() {
                                let diagx = point.0 + dir.0;
                                let diagy = point.1 + dir.1;
                                if diagx >= 0 && diagx < table.width() as i32 &&
                                diagy >= 0 && diagy < table.height() as i32 {
                                    if table.get_obstacle(diagx, diagy) == Obstacle::Platform {
                                        visible.insert((diagx, diagy));
                                    }
                                }

                            } 
                            visible.insert((newx, newy));
                        }
                    }
                }
            }
        }
    }

    visible
}