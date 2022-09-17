use util::{voronoi, vec_ops::neighbors};

use crate::{obstacle_table::ObstacleTable, obstacle::Obstacle};

pub fn apply_voronoi_inv(table: &mut ObstacleTable, nseeds: usize) {
    let vmembers = voronoi::voronoi_membership(nseeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors((x as i32, y as i32), (0, 0), (table.width() as i32 - 1, table.height() as i32 - 1));
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
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

pub fn apply_voronoi(table: &mut ObstacleTable, nseeds: usize) {
    let vmembers = voronoi::voronoi_membership(nseeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors((x as i32, y as i32), (0, 0), (table.width() as i32 - 1, table.height() as i32 - 1));
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
                let nbrseed = vmembers[nbr];
                if myseed.0 != nbrseed.0 || myseed.1 != nbrseed.1 {
                    nbrs_count += 1;
                }
            }

            if nbrs_count < 2 {
                table.set_obstacle((x as i32, y as i32), Obstacle::Platform(0));
            }
        }
    }
}

pub fn apply_voronoi_v2(table: &mut ObstacleTable, nseeds: usize) {
    let vmembers = voronoi::voronoi_membership(nseeds, table.width(), table.height());
    for x in 0..table.width() {
        for y in 0..table.height() {
            let mut nbrs_count = 0;
            let nbrs = neighbors((x as i32, y as i32), (0, 0), (table.width() as i32 - 1, table.height() as i32 - 1));
            let myseed = vmembers[&(x as i32, y as i32)];
            for nbr in nbrs.iter() {
                let nbrseed = vmembers[nbr];
                if myseed.0 != nbrseed.0 || myseed.1 != nbrseed.1 {
                    nbrs_count += 1;
                }
            }

            if nbrs_count < 3 {
                table.set_obstacle((x as i32, y as i32), Obstacle::Platform(0));
            }
        }
    }
}