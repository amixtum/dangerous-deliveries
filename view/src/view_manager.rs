use console_engine::{screen::Screen, Color};

use util::files;

use super::file_chooser_viewer;
use super::gameover_viewer;
use super::help_viewer;
use super::main_menu_viewer;
use super::main_viewer::MainViewer;
use super::options_viewer;
use super::youwin_viewer;

use model::goal_table::GoalTable;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::state::GameState;

pub struct ViewManager {
    main_view: MainViewer,
}

impl ViewManager {
    pub fn new() -> Self {
        ViewManager {
            main_view: MainViewer::new(64), // set log length to 64
        }
    }
}

impl ViewManager {
    pub fn get_screen(
        &mut self,
        state: &GameState,
        obs_table: &ObstacleTable,
        goal_table: &GoalTable,
        player: &Player,
        ai: &Vec<Player>,
        max_falls: u32,
        max_speed: f32,
        fallover_threshold: f32,
        window_width: u32,
        window_height: u32,
        current_lsystem: &str,
    ) -> Screen {
        match state {
            GameState::MainMenu => {
                return main_menu_viewer::main_menu_screen(window_width, window_height);
            }
            GameState::SizeChooser => {
                return options_viewer::size_chooser_viewer(window_width, window_height);
            }
            GameState::LSystemChooser(size_index) => {
                return file_chooser_viewer::file_chooser_screen(
                    window_width,
                    window_height,
                    &files::get_file_chooser_string(*size_index as u32),
                    current_lsystem,
                );
            }
            GameState::Help => {
                return help_viewer::help_screen(window_width, window_height);
            }
            GameState::GameOver => {
                self.main_view.clear_log();
                return gameover_viewer::game_over_screen(
                    obs_table,
                    &player,
                    window_width,
                    window_height,
                );
            }
            GameState::YouWin => {
                self.main_view.clear_log();
                return youwin_viewer::win_screen(&player, window_width, window_height);
            }
            GameState::Restart => {
                self.main_view.clear_log();
                return self.main_view.draw_layout(
                    &obs_table,
                    &goal_table,
                    &player,
                    ai,
                    max_falls,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
            GameState::LookMode => {
                self.main_view
                    .add_string(String::from("Look Where?"), Color::Yellow);
                return self.main_view.draw_layout(
                    &obs_table,
                    &goal_table,
                    &player,
                    ai,
                    max_falls,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
            GameState::LookedAt(s) => {
                self.main_view.add_string(String::from(s), Color::Green);
                return self.main_view.draw_layout(
                    &obs_table,
                    &goal_table,
                    &player,
                    ai,
                    max_falls,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
            GameState::DeliveredPackage(_, _) => {
                self.main_view
                    .add_string(String::from("Delivered"), Color::Blue);
                return self.main_view.draw_layout(
                    &obs_table,
                    &goal_table,
                    &player,
                    ai,
                    max_falls,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
            GameState::PostMove => {
                self.main_view
                    .add_message(&obs_table, &player, &player.recent_event);
                //self.main_view.debug(&player);
                return self.main_view.draw_layout(
                    &obs_table,
                    &goal_table,
                    &player,
                    ai,
                    max_falls,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
            GameState::Playing => {
                return self.main_view.draw_layout(
                    &obs_table,
                    &goal_table,
                    &player,
                    ai,
                    max_falls,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
        }
    }
}
