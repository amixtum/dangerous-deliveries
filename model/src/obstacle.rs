#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Obstacle {
    Platform,
    Pit,
    Rail(i32, i32),
    Wall,
}
