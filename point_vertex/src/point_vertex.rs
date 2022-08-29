use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PointVertex {
    pub x: u32,
    pub y: u32,
    pub height: u32,
}

impl PointVertex {
    pub fn new(x: u32, y: u32, height: u32) -> PointVertex {
        PointVertex {
            x,
            y,
            height,
        }
    }
}

impl Ord for PointVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.x * self.y) + self.x < (other.x * other.y) + other.x {
            return Ordering::Less
        }
        else if (self.x * self.y) + self.x == (other.x * other.y) + other.x {
            return Ordering::Equal
        }
        else {
            return Ordering::Greater
        }
    }
}

impl PartialOrd for PointVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.x * self.y) + self.x < (other.x * other.y) + other.x {
            return Some(Ordering::Less)
        }
        else if (self.x * self.y) + self.x == (other.x * other.y) + other.x {
            return Some(Ordering::Equal)
        }
        else {
            return Some(Ordering::Greater)
        }
    }
}

impl Hash for PointVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.height.hash(state);
    }
}
