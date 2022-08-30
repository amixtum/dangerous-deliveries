#[derive(Clone, Copy)]
pub struct Player {
    pub position: (i32, i32, i32),
    pub speed: (f32, f32),
    pub balance: (f32, f32),
}

impl Player {
    pub fn new(x: i32, y: i32, height: i32) -> Self {
        Player {
            position: (x, y, height),
            speed: (0.0, 0.0),
            balance: (0.0, 0.0),
        } 
    }

}

impl Player {

}

