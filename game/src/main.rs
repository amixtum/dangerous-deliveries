pub mod game;

use std::env;

use game::Game;

fn main() -> Result<(), String> {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    if args.len() != 8 {
        return Err(format!("Usage: <binary> <window_width> <window_height> <game_width> <game_height> <phsyics_conf> <lsystem_def> <turtle_conf>"));
    }

    if let Ok(window_width) = args[1].parse::<u32>() {
        if let Ok(window_height) = args[2].parse::<u32>() {
            if let Ok(game_width) = args[3].parse::<u32>() {
                if let Ok(game_height) = args[4].parse::<u32>() {
                    if let Ok(mut g) = Game::new(window_width,
                                          window_height,
                                          30,
                                          game_width,
                                          game_height,
                                          &args[5],
                                          &args[6],
                                          &args[7],) {
                        while g.run() {
                        }

                        return Ok(());
                    }
                }
            }
        }
    }

    Err(String::from("Could not parse args"))
}
