#[derive(Clone, Copy, PartialEq)]
pub enum ProcState {
    MainMenu,
    Help,
    GameOver,
    Playing,
    PostMove,
    Restart,
    Chat,
    GotPackage(i32, i32),
    DeliveredPackage,
    LookMode,
}
