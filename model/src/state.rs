#[derive(Clone)]
pub enum ProcState {
    MainMenu,
    Help,
    GameOver,
    Playing,
    PostMove,
    Restart,
    GotPackage(i32, i32),
    DeliveredPackage, // ???
    LookMode,
    LookedAt(String),
}
