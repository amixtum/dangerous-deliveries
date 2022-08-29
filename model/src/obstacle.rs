use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub x: usize,
    pub y: usize,
    pub height: Option<usize>,
    pub direction: Option<(f32, f32)>,
}

impl Obstacle {
    pub fn new(x: usize, y: usize, height: Option<usize>, direction: Option<(f32, f32)>) -> Self {
        Obstacle {
            x,
            y,
            height,
            direction
        }
    }
}

impl PartialOrd for Obstacle {
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

impl Ord for Obstacle {
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

impl PartialEq for Obstacle {
    fn eq(&self, other: &Self) -> bool {
        let l = self.x * self.y + self.x;
        let r = other.x * other.y + other.x;
        l == r
    }
}

impl Eq for Obstacle { }

impl Hash for Obstacle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state):
    }
}
