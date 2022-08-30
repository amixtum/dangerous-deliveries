use super::obstacle::Obstacle;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub obstacle: Obstacle,
}

impl Cell {
    pub fn new(x: i32, y: i32, obstacle: Obstacle) -> Self {
        Cell {
            x,
            y,
            obstacle,
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let l = self.x * self.y + self.x;
        let r = other.x * other.y + other.x;
        if l < r {
            Some(Ordering::Less)
        }
        else if l == r {
            Some(Ordering::Equal)
        }
        else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        let l = self.x * self.y + self.x;
        let r = other.x * other.y + other.x;
        if l < r {
            Ordering::Less
        }
        else if l == r {
            Ordering::Equal
        }
        else {
            Ordering::Greater
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(&other) == Ordering::Equal;
    }
}

impl Eq for Cell { }

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
