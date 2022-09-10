#[derive(Clone)]
pub enum GameState {
    MainMenu,
    //Options,
    LSystemChooser(i32),
    SizeChooser,
    //ModelChooser,
    Help,
    GameOver,
    YouWin,
    Playing,
    PostMove,
    Restart,
    DeliveredPackage(i32, i32),
    LookMode,
    LookedAt(String),
}
