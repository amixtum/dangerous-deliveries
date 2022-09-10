use console_engine::{pixel, screen::Screen, Color};

use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;

pub fn game_over_screen(
    obs_table: &ObstacleTable,
    player: &Player,
    width: u32,
    height: u32,
) -> Screen {
    let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));
    screen.print_fbg(
        (width as i32 / 2) - "All Packages Delivered".chars().count() as i32 / 2,
        (height as i32 / 2) - 1,
        "All Packages Delivered",
        Color::Green,
        Color::Black,
    );

    let mut info = String::new();
    if let PlayerEvent::GameOver(time) = player.recent_event {
        info.push_str(&format!(
            "Score: {}",
            (((obs_table.width() as f32 * obs_table.height() as f32) / time as f32)
                * player.n_delivered as f32)
                .round() as i32
        ));
    } else {
        info.push_str(&format!("You delivered {} packages", player.n_delivered));
    }
    screen.print(
        (width as i32 / 2) - info.chars().count() as i32 / 2,
        height as i32 / 2,
        &info,
    );

    info.clear();

    info.push_str("Press R to restart. Press Esc to exit.");
    screen.print(
        (width as i32 / 2) - info.chars().count() as i32 / 2,
        (height as i32 / 2) + 1,
        &info,
    );

    screen
}
