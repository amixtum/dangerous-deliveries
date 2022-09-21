use std::collections::{HashSet};

use rltk::Point;

use crate::{obstacle_table::ObstacleTable};

pub fn get_fov(
    center: (i32, i32),
    table: &ObstacleTable,
    radius: i32
) -> HashSet<Point> {
    rltk::field_of_view_set(Point::new(center.0, center.1), radius, table)
}








/*
pub fn get_visible(
    center: (i32, i32),
    table: &ObstacleTable,
    diameter: u32,
) -> HashSet<(i32, i32)> {
    let mut visible = HashSet::new();
    let mut tries = 0;

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let diags = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut iters = HashMap::new();
    iters.insert((0, -1), HashSet::new());
    iters.insert((0, 1), HashSet::new());
    iters.insert((-1, 0), HashSet::new());
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
    let mut iters_done = |iters: &HashMap<(i32, i32), HashSet<(i32, i32)>>| {
        if tries >= diameter * directions.len() as u32 {
            return true;
        }

        tries += 1;
        for iter in iters.iter() {
            if iter.1.len() < diameter as usize {
                return false;
            }
        }
        return true;
    };

    while !iters_done(&iters) {
        for dir in directions.iter() {
            let set = iters
                .get(&dir)
                .unwrap()
                .iter()
                .map(|p| *p)
                .collect::<Vec<_>>();
            for point in set.iter() {
                let newx = point.0 + dir.0;
                let newy = point.1 + dir.1;
                for dir in directions.iter() {
                    if newx >= 0
                        && newx < table.width() as i32
                        && newy >= 0
                        && newy < table.height() as i32
                    {
                        if table.get_obstacle(newx, newy) == Obstacle::Platform {
                            if iters.get_mut(&dir).unwrap().insert((newx, newy)) {
                                visible.insert((newx, newy));
                            }
                        } else {
                            for dir in diags.iter() {
                                let diagx = point.0 + dir.0;
                                let diagy = point.1 + dir.1;
                                if diagx >= 0
                                    && diagx < table.width() as i32
                                    && diagy >= 0
                                    && diagy < table.height() as i32
                                {
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
*/
