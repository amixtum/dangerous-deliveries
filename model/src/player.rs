#[derive(Clone, Copy)]
pub struct Player {
    pub position: (usize, usize, usize),
    pub speed: (f32, f32),
    pub balance: (f32, f32),
}

impl Player {
    pub fn new(x: usize, y: usize, height: usize) -> Self {
        Player {
            position: (x, y, height),
            speed: (0.0, 0.0),
            balance: (0.0, 0.0),
        } 
    }
}

