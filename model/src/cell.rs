use super::player::Player;
use super::obstacle::Obstacle;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Cell {
    pub player: Option<Player>,
    pub obstacle: Obstacle,
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.obstacle.partial_cmp(&other.obstacle)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.obstacle.cmp(&other.obstacle)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.obstacle.eq(&other.obstacle) 
    }
}

impl Eq for Cell { }

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.obstacle.hash(state);
    }
}
