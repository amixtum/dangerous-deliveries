use std::{collections::{HashSet, HashMap}, hash::Hash};

use rand::Rng;

use crate::vec_ops;

pub fn voronoi_seeds(n: usize, width: u32, height: u32) -> HashSet<(i32, i32)> {
    let mut seeds = HashSet::new();

    if n >= width as usize * height as usize {
        return seeds;
    }

    while seeds.len() < n {
        let x = rand::thread_rng().gen_range(0..width) as i32;
        let y = rand::thread_rng().gen_range(0..height) as i32;
        seeds.insert((x, y));
    }

    seeds
}

pub fn voronoi_membership(seeds: &HashSet<(i32, i32)>, width: u32, height: u32) -> HashMap<(i32, i32), (i32, i32)> {
    let mut voronoi_dist = vec![((0, 0), 0.0f32); seeds.len()];
    let mut voronoi_membership = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            for (seed, point) in seeds.iter().enumerate() {
                let dist = vec_ops::magnitude((x as f32 - point.0 as f32, y as f32 - point.1 as f32));
                voronoi_dist[seed] = (*point, dist);
            }

            voronoi_dist.sort_by(|l, r| {
                l.1.partial_cmp(&r.1).unwrap()
            });

            voronoi_membership.insert((x as i32, y as i32), voronoi_dist[0].0);
        }
    }

    voronoi_membership
}