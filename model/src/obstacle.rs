#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Obstacle {
    Platform,
    Pit,
    Rail(i32, i32),
    Wall,
}
