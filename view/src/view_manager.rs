use controller::ai_controller::AIController;
use controller::player_controller::PlayerController;

use super::gameover_viewer;
use super::help_viewer;
use super::main_menu_viewer;
use super::main_viewer::MainViewer;

use model::goal_table::GoalTable;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::state::ProcState;

pub struct ViewManager {
    pub main_view: MainViewer,
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
        ctx: &mut rltk::Rltk,
        state: &ProcState,
        obs_table: &mut ObstacleTable,
        goal_table: &GoalTable,
        player: &Player,
        ai: &Vec<AIController>,
        controller: &PlayerController,
        max_speed: f32,
        fallover_threshold: f32,
        window_width: u32,
        window_height: u32,
    ) {
        match state {
            ProcState::MainMenu => {
                main_menu_viewer::main_menu_screen(ctx, window_width, window_height);
            }
            ProcState::Help => {
                help_viewer::help_screen(ctx, window_width, window_height);
            }
            ProcState::GameOver => {
                self.main_view.clear_log();
                gameover_viewer::game_over_screen(
                    ctx,
                    obs_table,
                    player,
                    window_width,
                    window_height,
                );
            }
            ProcState::Playing
            | ProcState::PostMove
            | ProcState::GotPackage(..)
            | ProcState::LookedAt(_)
            | ProcState::LookMode
            | ProcState::Restart 
            | ProcState::DeliveredPackage => {
                //self.main_view.clear_log();

                return self.main_view.draw_layout(
                    ctx,
                    obs_table,
                    goal_table,
                    player,
                    ai,
                    controller,
                    max_speed,
                    fallover_threshold,
                    window_width,
                    window_height,
                );
            }
        }
    }
}
