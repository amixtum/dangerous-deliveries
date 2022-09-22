use petgraph::{unionfind::UnionFind};
use rltk::{Algorithm2D, Point, BaseMap, DistanceAlg};
use util::vec_ops;

use crate::player::Player;

use super::obstacle::Obstacle;
use super::traversability::Traversability;

use std::collections::{HashMap, HashSet};

pub struct ObstacleTable {
    width: u32,
    height: u32,
    table: Vec<Vec<Obstacle>>,
    pub platforms: Vec<(i32, i32)>,
    pub ufind: UnionFind<u32>,
    pub blocked: HashMap<(i32, i32), Player>,
    pub revelead: HashSet<(i32, i32)>,
}

impl BaseMap for ObstacleTable {
    fn is_opaque(&self, idx: usize) -> bool {
        let pt = self.index_to_point2d(idx);
        self.get_obstacle(pt.x, pt.y) != Obstacle::Platform ||
        self.blocked.contains_key(&(pt.x, pt.y)) 
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width as i32;
        let y = idx as i32 / self.width as i32;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };
        if self.is_exit_valid(x-1, y-1) { exits.push((idx-1-w, 1.45)) };
        if self.is_exit_valid(x+1, y-1) { exits.push((idx+1-w, 1.45)) };
        if self.is_exit_valid(x-1, y+1) { exits.push((idx-1+w, 1.45)) };
        if self.is_exit_valid(x+1, y+1) { exits.push((idx+1+w, 1.45)) };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);
        DistanceAlg::Manhattan.distance2d(p1, p2)
    }
}

impl Algorithm2D for ObstacleTable {
    fn dimensions(&self) -> rltk::Point {
        Point::new(self.width, self.height)
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        Point::new(idx % self.width as usize, idx / self.width as usize)
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        (pt.y * self.width as i32 + pt.x) as usize
    }
}

impl ObstacleTable {
    pub fn new(width: u32, height: u32) -> Self {
        let mut ct = ObstacleTable {
            width,
            height,
            table: Vec::new(),
            platforms: Vec::new(),
            blocked: HashMap::new(),
            revelead: HashSet::new(),
            ufind: UnionFind::new(width as usize * height as usize),
        };

        for x in 0..width {
            ct.table.push(Vec::new());
            for _ in 0..height {
                ct.table[x as usize].push(Obstacle::Platform);
            }
        }

        //ct.lsystem.update_n(ct.lsystem.iterations);

        ct.regen_table();

        ct
    }
}

impl ObstacleTable {
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 0 || x > self.width as i32 - 1 || y < 0 || y > self.height as i32 - 1 {
            return false;
        }
        self.get_obstacle(x, y) == Obstacle::Platform
    }

    pub fn xy_flat(&self, x: i32, y: i32) -> u32 {
        y as u32 * self.width + x as u32
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_obstacle(&self, x: i32, y: i32) -> Obstacle {
        self.table[x as usize][y as usize]
    }

    pub fn get_direction(&self, x: i32, y: i32) -> Option<(f32, f32)> {
        match self.table[x as usize][y as usize] {
            Obstacle::Platform => None,
            Obstacle::Pit => None,
            Obstacle::Rail(xdir, ydir) => Some((xdir as f32, ydir as f32)),
            Obstacle::Wall => None,
        }
    }

    pub fn can_traverse(&self, (from_x, from_y): (i32, i32), (to_x, to_y): (i32, i32)) -> bool {
        let x_diff = to_x - from_x;
        let y_diff = to_y - from_y;

        if (self.blocked.contains_key(&(to_x, to_y)) && !(from_x == to_x && from_y == to_y)) || self.get_obstacle(to_x, to_y) == Obstacle::Wall {
            return false;
        }

        if to_x >= 0 && to_x < self.width as i32 && to_y >= 0 && to_y < self.height as i32 {
            return x_diff.abs() <= 1 && y_diff.abs() <= 1;
        }

        false
    }

    pub fn traversability(
        &self,
        (from_x, from_y): (i32, i32),
        (to_x, to_y): (i32, i32),
    ) -> Traversability {
        let x_diff = to_x as i32 - from_x as i32;
        let y_diff = to_y as i32 - from_y as i32;

        if to_x >= 0
            && to_x < self.width as i32
            && to_y >= 0
            && to_y < self.height as i32
            && x_diff.abs() <= 1
            && y_diff.abs() <= 1
        {
            return Traversability::Flat;
        }

        return Traversability::No;
    }

    pub fn update_platforms(&mut self) {
        self.platforms.clear();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_obstacle(x as i32, y as i32) == Obstacle::Platform {
                    self.platforms.push((x as i32, y as i32));
                }
            }
        }
    }

    pub fn compute_unions(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_obstacle(x as i32, y as i32) == Obstacle::Platform {
                    let nbrs = vec_ops::neighbors((x as i32, y as i32), (0, 0), (self.width as i32 - 1, self.height as i32 - 1));
                    for nbr in nbrs.iter() {
                        if self.get_obstacle(nbr.0, nbr.1) == Obstacle::Platform {
                            self.ufind.union(self.xy_flat(x as i32, y as i32), self.xy_flat(nbr.0, nbr.1));
                        }
                    }
                }
            }
        } 
    }

    pub fn set_obstacle(&mut self, (x, y): (i32, i32), obs: Obstacle) {
        self.table[x as usize][y as usize] = obs;
    }

    // assumes an obstacle already exists at x, y and
    // and copies its height to the platform
    // if it is a pit, it gets height 0
    pub fn set_platform(&mut self, (x, y): (i32, i32)) {
        match self.table[x as usize][y as usize] {
            Obstacle::Pit => {
                self.table[x as usize][y as usize] = Obstacle::Platform;
            }
            _ => {
                self.table[x as usize][y as usize] = Obstacle::Platform;
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.table.clear();
        for x in 0..width {
            self.table.push(Vec::new());
            for _ in 0..height {
                self.table[x as usize].push(Obstacle::Platform);
            }
        }
        self.regen_table();
    }

    pub fn regen_table(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.table[x as usize][y as usize] = Obstacle::Platform;
            }
        }
    }
}
