pub mod game;

use std::env;

use game::Game;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    if let Ok(mut g) = Game::new(80,
                          20,
                          30,
                          80,
                          80,
                          &args[1]) {
        while g.run() {
        }
    }
}
