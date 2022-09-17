#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerEvent {
    Wait,
    Move,
    OnRail,
    OffRail,
    FallOver,
    GameOver(i32),
}
