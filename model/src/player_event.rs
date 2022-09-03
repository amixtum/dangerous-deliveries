#[derive(Debug, Clone, Copy)]
pub enum PlayerEvent {
    Wait,
    Move,
    OnRail,
    OffRail,
    FallOver,
    GameOver(i32),
}
