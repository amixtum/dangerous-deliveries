pub mod game;

use std::fs;

use std::env;

use project_root;

use game::Game;

fn main() -> Result<(), String> {
    env::set_var("RUST_BACKTRACE", "1");
    if let Ok(mut game) = load_game() {
        while game.run() {
        }
        return Ok(());
    }

    Err(String::from("Could not load game"))
}

fn load_game() -> Result<Game, String> {
    let mut window_width = 80;
    let mut window_height = 20;
    let mut game_width = 80;
    let mut game_height = 20;
    let mut lsystem_path = "";
    let mut table_path = "";
    let mut model_path = "";
    let mut game_conf_path = "";

    if let Ok(pr) = project_root::get_project_root() {
        let mut path = String::from("");
        match pr.to_str() {
            Some(s) => {
                path.clear();
                path.push_str(s);
            },
            None => { },
        }
        if let Ok(contents) = fs::read_to_string(format!("{}/config/{}", path, "window.txt")) {
            for line in contents.lines() {
                if let Some(c) = line.chars().nth(0) {
                    if c == '#' {
                        continue;
                    }
                }
                else {
                    continue;
                }
                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                if words.len() == 2 {
                    if words[0] == "window_width" {
                        if let Ok(num) = words[1].parse::<u32>() {
                            window_width = num; 
                        }
                    }
                    else if words[0] == "window_height" {
                        if let Ok(num) = words[1].parse::<u32>() {
                            window_height = num; 
                        }
                    }
                    else if words[0] == "game_width" {
                        if let Ok(num) = words[1].parse::<u32>() {
                            game_width = num; 
                        }
                    }
                    else if words[0] == "game_height" {
                        if let Ok(num) = words[1].parse::<u32>() {
                            game_height = num; 
                        }
                    }
                    else if words[0] == "lsystem" {
                        lsystem_path = &words[1]; 
                    }
                    else if words[0] == "model" {
                        model_path = &words[1]; 
                    }
                    else if words[0] == "table" {
                        table_path = &words[1]; 
                    }
                    else if words[0] == "game" {
                        game_conf_path = &words[1]; 
                    }
                }
            }
            if let Ok(g) = Game::new(window_width,
                                     window_height,
                                     30, // 30 fps
                                     game_width,
                                     game_height,
                                     &format!("{}/config/{}", path, game_conf_path),
                                     &format!("{}/config/{}", path, model_path),
                                     &format!("{}/config/{}", path, lsystem_path),
                                     &format!("{}/config/{}", path, table_path)) {
                return Ok(g);
            }
        } 
    }

    Err(String::from("Could not create game"))
}
