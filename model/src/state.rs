#[derive(Clone)]
pub enum GameState {
    MainMenu,
    //Options,
    //LSystemChooser,
    //SizeChooser,
    //ModelChooser,
    Help,
    GameOver,
    YouWin,
    Playing,
    PostMove,
    Restart,
    DeliveredPackage,
    LookMode,
    LookedAt(String),
}


