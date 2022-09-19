#[derive(Clone)]
pub enum ProcState {
    MainMenu,
    Help,
    GameOver,
    Playing,
    PostMove,
    Restart,
    DeliveredPackage(i32, i32),
    LookMode,
    LookedAt(String),
}
