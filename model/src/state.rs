#[derive(Clone)]
pub enum GameState {
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
