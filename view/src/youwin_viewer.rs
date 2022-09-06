use console_engine::{
    pixel,
    Color,
    screen::Screen
};

use model::player::Player;
use model::player_event::PlayerEvent;

pub fn win_screen(player: &Player, width: u32, height: u32) -> Screen {
    let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));
    screen.print_fbg((width as i32 / 2) - "You Win".chars().count() as i32 / 2, (height as i32 / 2) - 1, "You Win", Color::Green, Color::Black);

    let mut info = String::new();
    if let PlayerEvent::GameOver(time) = player.recent_event {
        info.push_str(&format!("Time: {}", time));
        screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, height as i32 / 2, &info);
    } 

    info.clear(); 

    info.push_str("Press R to restart. Press Esc to exit.");
    screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, (height as i32 / 2) + 1, &info);

    screen
}