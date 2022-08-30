#[derive(Clone, Copy)]
pub enum Obstacle {
    Platform(i32),
    Pit,
    Rail(i32, (f32, f32)),
}
