#[derive(Clone, Copy)]
pub enum Obstacle {
    Platform(i32),
    Pit,
    Rail(i32, (f32, f32)),
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObstacleType {
    Platform,
    Pit,
    Rail(i32, i32),
    Wall,
}
