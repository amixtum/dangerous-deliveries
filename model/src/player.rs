use super::player_event::PlayerEvent;

#[derive(Clone, Copy)]
pub struct Player {
    pub position: (i32, i32, i32),
    pub speed: (f32, f32),
    pub balance: (f32, f32),
    pub time: f32,
    pub recent_event: PlayerEvent,

    pub n_falls: i32,
    pub n_delivered: u32,
}

#[derive(Clone, Copy)]
pub enum PlayerType {
    Human,
    AI,
}

impl Player {
    pub fn new(x: i32, y: i32, height: i32) -> Self {
        Player {
            position: (x, y, height),
            speed: (0.0, 0.0),
            balance: (0.0, 0.0),
            time: 0.0,
            recent_event: PlayerEvent::Wait,
            n_falls: 0,
            n_delivered: 0,
        }
    }
}

impl Player {
    pub fn x(&self) -> i32 {
        self.position.0
    }
    pub fn y(&self) -> i32 {
        self.position.1
    }
    pub fn height(&self) -> i32 {
        self.position.2
    }

    pub fn xy(&self) -> (i32, i32) {
        (self.x(), self.y())
    }

    pub fn speed_x(&self) -> f32 {
        self.speed.0
    }
    pub fn speed_y(&self) -> f32 {
        self.speed.1
    }

    pub fn balance_x(&self) -> f32 {
        self.balance.0
    }
    pub fn balance_y(&self) -> f32 {
        self.balance.1
    }
}
