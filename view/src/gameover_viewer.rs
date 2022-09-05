use console_engine::{
    pixel,
    Color,
    screen::Screen
};

use model::goal_table::GoalTable;
use model::player::Player;
use model::player_event::PlayerEvent;

pub fn game_over_screen(goals: &GoalTable, player: &Player, max_goals: u32, width: u32, height: u32) -> Screen {
    let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));
    screen.print_fbg((width as i32 / 2) - "Game Over".chars().count() as i32 / 2, (height as i32 / 2) - 1, "Game Over", Color::Red, Color::Black);

    let mut info = String::new();
    if let PlayerEvent::GameOver(time) = player.recent_event {
        info.push_str(&format!("Time: {}, Packages Delivered: {}", time, max_goals as i32 - goals.count() as i32));
    } 
    else {
        info.push_str(&format!("Packages Delivered: {}", max_goals as i32 - goals.count() as i32));
    }
    screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, height as i32 / 2, &info);

    info.clear(); 

    info.push_str("Press R to restart. Press Esc to exit.");
    screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, (height as i32 / 2) + 1, &info);

    screen
}
