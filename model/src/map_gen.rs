use std::{collections::HashSet, f32::consts::PI};

use rand::Rng;
use util::{
    vec_ops::{self, neighbors},
    voronoi,
};

use crate::{
    goal_table::GoalTable, obstacle::Obstacle, obstacle_automata, obstacle_table::ObstacleTable,
};

pub fn tunnel_position(table: &mut ObstacleTable, (x, y): (i32, i32)) {
    let mut directions = Vec::new();
    directions.push((0, -1));
    directions.push((0, 1));
    directions.push((-1, 0));
    directions.push((1, 0));
    directions.push((-1, -1));
    directions.push((1, -1));
    directions.push((-1, 1));
    directions.push((1, 1));

    let mut iters = vec![(x, y); directions.len()];
    let mut one_done = false;
    let mut tunneler1: (i32, i32) = (-1, -1);
    let mut tunneler2: (i32, i32);
    let mut main_direction1: (i32, i32) = (0, 0);
    let main_direction2: (i32, i32);
    'outer: loop {
        for dir in directions.iter().enumerate() {
            iters[dir.0].0 += dir.1 .0;
            iters[dir.0].1 += dir.1 .1;
            if iters[dir.0].0 <= 0
                || iters[dir.0].0 >= table.width() as i32
                || iters[dir.0].1 <= 0
                || iters[dir.0].1 >= table.height() as i32
            {
                continue;
            }
            let nbrs = neighbors(
                (iters[dir.0].0, iters[dir.0].1),
                (0, 0),
                (table.width() as i32 - 1, table.height() as i32 - 1),
            );
            if table.get_obstacle(iters[dir.0].0, iters[dir.0].1) == Obstacle::Platform {
                let mut count_platforms = 0;
                for nbr in nbrs {
                    if table.get_obstacle(nbr.0, nbr.1) == Obstacle::Platform {
                        count_platforms += 1;
                    }
                }

                if count_platforms > 1 {
                    if !one_done {
                        tunneler1 = (iters[dir.0].0, iters[dir.0].1);

                        // the opposite direction of how we got there
                        main_direction1 = (-dir.1 .0, -dir.1 .1);
                        one_done = true;
                    } else {
                        tunneler2 = (iters[dir.0].0, iters[dir.0].1);

                        // the opposite direction of how we got there
                        main_direction2 = (-dir.1 .0, -dir.1 .1);
                        break 'outer;
                    }
                }
            }
        }
    }
    while !(tunneler1.0 == x && tunneler1.1 == y) && !(tunneler2.0 == x && tunneler2.1 == y) {
        let mut sign: i32 = rand::thread_rng().gen_range(-1..=1);
        if sign == 0 {
            sign = -1 + (rand::thread_rng().gen_range(0..=1) % 2) * 2;
        }
        let double1 = vec_ops::rotate(main_direction1, sign as f32 * PI / 8.0);

        sign = rand::thread_rng().gen_range(-1..=1);
        if sign == 0 {
            sign = -1 + (rand::thread_rng().gen_range(0..=1) % 2) * 2;
        }
        let double2 = vec_ops::rotate(main_direction2, sign as f32 * PI / 8.0);

        let mut tunnel1 = (
            tunneler1.0 + main_direction1.0,
            tunneler1.1 + main_direction1.1,
        );
        tunnel1.0 = tunnel1.0.clamp(0, table.width() as i32 - 1);
        tunnel1.1 = tunnel1.1.clamp(0, table.height() as i32 - 1);

        let mut perturb_tunnel1 = (tunneler1.0 + double1.0, tunneler1.1 + double1.1);
        perturb_tunnel1.0 = perturb_tunnel1.0.clamp(0, table.width() as i32 - 1);
        perturb_tunnel1.1 = perturb_tunnel1.1.clamp(0, table.height() as i32 - 1);

        let mut tunnel2 = (
            tunneler2.0 + main_direction2.0,
            tunneler2.1 + main_direction2.1,
        );
        tunnel2.0 = tunnel2.0.clamp(0, table.width() as i32 - 1);
        tunnel2.1 = tunnel2.1.clamp(0, table.height() as i32 - 1);

        let mut perturb_tunnel2 = (tunneler2.0 + double2.0, tunneler2.1 + double2.1);
        perturb_tunnel2.0 = perturb_tunnel2.0.clamp(0, table.width() as i32 - 1);
        perturb_tunnel2.1 = perturb_tunnel2.1.clamp(0, table.height() as i32 - 1);

        table.set_obstacle(tunnel1, Obstacle::Platform);
        table.set_obstacle(perturb_tunnel1, Obstacle::Platform);
        tunneler1 = tunnel1;

        table.set_obstacle(tunnel2, Obstacle::Platform);
        table.set_obstacle(perturb_tunnel2, Obstacle::Platform);
        tunneler2 = tunnel2;
    }
    while !(tunneler2.0 == x && tunneler2.1 == y) {
        let mut sign = rand::thread_rng().gen_range(-1..=1);
        if sign == 0 {
            sign = -1 + (rand::thread_rng().gen_range(0..=1)) * 2;
        }
        let double2 = vec_ops::rotate(main_direction2, sign as f32 * PI / 8.0);

        let tunnel2 = (
            tunneler2.0 + main_direction2.0,
            tunneler2.1 + main_direction2.1,
        );
        //tunnel2.0 = tunnel2.0.clamp(0, table.width() as i32 - 1);
        //tunnel2.1 = tunnel2.1.clamp(0, table.height() as i32 - 1);

        let mut perturb_tunnel2 = (tunneler2.0 + double2.0, tunneler2.1 + double2.1);
        perturb_tunnel2.0 = perturb_tunnel2.0.clamp(0, table.width() as i32 - 1);
        perturb_tunnel2.1 = perturb_tunnel2.1.clamp(0, table.height() as i32 - 1);

        table.set_obstacle(tunnel2, Obstacle::Platform);
        table.set_obstacle(perturb_tunnel2, Obstacle::Platform);
        tunneler2 = tunnel2;
    }
}

// tunnels into the nearest open space
pub fn tunnel_goals(table: &mut ObstacleTable, goals: &GoalTable) {
    for goal in goals.goals() {
        tunnel_position(table, *goal);
    }
}

pub fn voronoi_mapgen(obs_table: &mut ObstacleTable, goals: &GoalTable) {
    let a = obs_table.width() / 6;
    let b = obs_table.height() / 6;

    for x in 0..obs_table.width() {
        for y in 0..obs_table.height() {
            obs_table.set_obstacle((x as i32, y as i32), Obstacle::Wall);
        }
    }

    let seeds = voronoi::voronoi_seeds(
        a as usize * b as usize,
        obs_table.width(),
        obs_table.height(),
    );
    for _ in 0..1 {
        apply_voronoi(obs_table, &seeds);
        //apply_voronoi_n2(obs_table, &seeds);
    }

    obstacle_automata::apply_automata(obs_table);
    //obstacle_automata::apply_automata(obs_table);
    tunnel_goals(obs_table, goals);
}

pub fn apply_voronoi_inv(table: &mut ObstacleTable, seeds: &HashSet<(i32, i32)>) {
    let vmembers = voronoi::voronoi_membership(seeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors(
                (x as i32, y as i32),
                (0, 0),
                (table.width() as i32 - 1, table.height() as i32 - 1),
            );
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
                if nbr.0.abs() == 1 && nbr.1.abs() == 1 {
                    continue;
                }

                let nbrseed = vmembers[nbr];
                if myseed.0 != nbrseed.0 || myseed.1 != nbrseed.1 {
                    nbrs_count += 1;
                }
            }

            if nbrs_count < 2 {
                table.set_obstacle((x as i32, y as i32), Obstacle::Wall);
            }
        }
    }
}

pub fn apply_voronoi_inv_n2(table: &mut ObstacleTable, seeds: &HashSet<(i32, i32)>) {
    let vmembers = voronoi::voronoi_membership(seeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors(
                (x as i32, y as i32),
                (0, 0),
                (table.width() as i32 - 1, table.height() as i32 - 1),
            );
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
                let nbrseed = vmembers[nbr];
                if myseed.0 != nbrseed.0 || myseed.1 != nbrseed.1 {
                    nbrs_count += 1;
                }
            }

            if nbrs_count % 2 == 1 && nbrs_count < 4 {
                table.set_obstacle((x as i32, y as i32), Obstacle::Wall);
            }
        }
    }
}

pub fn apply_voronoi(table: &mut ObstacleTable, seeds: &HashSet<(i32, i32)>) {
    let vmembers = voronoi::voronoi_membership(seeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors(
                (x as i32, y as i32),
                (0, 0),
                (table.width() as i32 - 1, table.height() as i32 - 1),
            );
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
                if nbr.0.abs() == 1 && nbr.1.abs() == 1 {
                    continue;
                }
                let nbrseed = vmembers[nbr];
                if myseed.0 != nbrseed.0 || myseed.1 != nbrseed.1 {
                    nbrs_count += 1;
                }
            }

            if nbrs_count < 2 {
                table.set_obstacle((x as i32, y as i32), Obstacle::Platform);
            }
        }
    }
}

pub fn apply_voronoi_n2(table: &mut ObstacleTable, seeds: &HashSet<(i32, i32)>) {
    let vmembers = voronoi::voronoi_membership(seeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors(
                (x as i32, y as i32),
                (0, 0),
                (table.width() as i32 - 1, table.height() as i32 - 1),
            );
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
                let nbrseed = vmembers[nbr];
                if myseed.0 != nbrseed.0 || myseed.1 != nbrseed.1 {
                    nbrs_count += 1;
                }
            }

            if nbrs_count < 5 && nbrs_count % 2 == 1 {
                table.set_obstacle((x as i32, y as i32), Obstacle::Platform);
            }
        }
    }
}
