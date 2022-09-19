use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;
use rltk::RGB;

pub fn game_over_screen(
    ctx: &mut rltk::Rltk,
    obs_table: &ObstacleTable,
    player: &Player,
    _width: u32,
    height: u32,
) {
    ctx.print_color_centered(
        (height as i32 / 2) - 1,
        RGB::named(rltk::GREEN),
        RGB::named(rltk::BLACK),
        "All Packages Delivered",
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
    ctx.print_centered(height as i32 / 2, &info);

    info.clear();

    info.push_str("Press R to restart. Press Esc to exit.");
    ctx.print_centered((height as i32 / 2) + 1, &info);
}
