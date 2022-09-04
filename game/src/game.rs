use console_engine::{ConsoleEngine, screen::Screen, KeyCode};

use super::state::GameState;
use model::cell_table::CellTable;
use model::player::Player;
use model::player_event::PlayerEvent;
use view::game_viewer::GameViewer;
use controller::player_controller::PlayerController;
use controller::look_mode::LookMode;

pub struct Game {
    table: CellTable,
    viewer: GameViewer,
    player_control: PlayerController,
    lookmode: LookMode,
    player: Player,

    engine: ConsoleEngine,
    window_width: u32,
    window_height: u32,

    redraw: bool,
    has_drawn: bool,
    lookmode_on: bool,
    helpscreen_on: bool,
    gameover: bool,
    youwin: bool,
    gameover_done: bool,

    state: GameState,
}

impl Game {
    pub fn new(window_width: u32, 
               window_height: u32, 
               target_fps: u32,
               table_width: u32, 
               table_height: u32, 
               conf_file: &str,
               lsystem_file: &str,
               turtle_file: &str,) -> Result<Self, String> {
        if let Ok(engine) = ConsoleEngine::init(window_width, window_height, target_fps) {
            return Ok(Game {
                table: CellTable::new(table_width, table_height, lsystem_file, turtle_file),
                viewer: GameViewer::new(64), // setting log length here, will specialize if needed
                player_control: PlayerController::new(conf_file),
                lookmode: LookMode::new(),
                player: Player::new(table_width as i32 / 2, table_height as i32 / 2, 0),

                engine,
                window_width,
                window_height,

                redraw: true,
                has_drawn: false,

                lookmode_on: false,

                helpscreen_on: false,

                gameover: false,
                youwin: false,
                gameover_done: false,

                state: GameState::MainMenu,
            });
        }
        Err(format!("Could not create window of width {}, height {}, at target_fps {}", window_width, window_height, target_fps))
    }
}

impl Game {
    pub fn run(&mut self) -> bool {
        self.engine.wait_frame();

        self.handle_input(); 

        if !self.has_drawn {
            self.engine.clear_screen();
            self.draw();
            self.engine.draw();
            self.has_drawn = true;
        }

        if self.redraw {
            self.engine.clear_screen();
            self.draw();
            self.engine.draw();
        }

        if self.engine.is_key_pressed(KeyCode::Esc) {
            return false;
        }

        true
    }

    pub fn handle_input(&mut self) {
        self.redraw = false;
        if self.gameover {
            if !self.gameover_done {
                self.table.regen_table();
                self.player = PlayerController::reset_player(&self.table, &self.player);
                self.viewer.clear_log();
                self.gameover_done = true;
            }

            if self.engine.is_key_pressed(KeyCode::Char('r')) {
                self.gameover = false;
                self.gameover_done = false;
                self.youwin = false;
                self.redraw = true;
            }
        } 
        else {
            if self.engine.is_key_pressed(KeyCode::Char(';')) {
                self.viewer.add_string(String::from("Look where?"));
                self.redraw = true;
                self.lookmode_on = true; 
            }
            else if self.engine.is_key_pressed(KeyCode::Enter) {
                self.table.regen_table();
                self.player = PlayerController::reset_player(&self.table, &self.player);
                self.viewer.clear_log();
                self.redraw = true;
            }
            else if self.engine.is_key_pressed(KeyCode::Char('0')) {
                self.helpscreen_on = !self.helpscreen_on;
                self.redraw = true;
            }

            if self.lookmode_on {
                for keycode in self.lookmode.get_keys()  {
                     if self.engine.is_key_pressed(*keycode) {
                        let result = self.lookmode.describe_direction(&self.table, &self.player, *keycode);
                        self.viewer.add_string(result);
                        self.redraw = true;
                        self.lookmode_on = false;
                        break;
                     }
                }
            }
            else {
                for keycode in self.player_control.get_keys() {
                    if self.engine.is_key_pressed(*keycode) {
                        let result = self.player_control.move_player(&self.table, &self.player, *keycode);

                        self.viewer.add_message(&self.table, &result, &result.recent_event);

                        self.player = result;

                        if self.table.remove_goal_if_reached(&self.player) {
                            self.viewer.add_string(String::from("Delivered"));
                        }

                        if let PlayerEvent::GameOver(_) = self.player.recent_event {
                            self.gameover = true;
                        } else if let PlayerEvent::FallOver = self.player.recent_event {
                            self.table.inc_fallover();
                            if self.table.check_falls() {
                                self.gameover = true;
                                self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                            }
                        }

                        if self.table.get_goals().len() <= 0 {
                            self.gameover = true;
                            self.youwin = true;
                            self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                        }

                        self.redraw = true;

                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        let screen: Screen;

        if self.gameover {
            if self.youwin {
                screen = self.viewer.win_screen(&self.player, self.window_width, self.window_height);
            }
            else {
                screen = self.viewer.game_over_screen(&self.table, &self.player, self.window_width, self.window_height);
            }
        }
        else if self.helpscreen_on {
            screen = self.viewer.help_screen(self.window_width, self.window_height);
        }
        else {
            screen = self.viewer.draw_layout(&self.table, 
                                             &self.player, 
                                             self.player_control.max_speed, 
                                             self.player_control.fallover_threshold, 
                                             self.window_width, 
                                             self.window_height);
        }
        self.engine.print_screen(0, 0, &screen);
    }

    fn get_screen(&self) -> Screen {
        match self.state {
            // GameState::MainMenu => { // TODO },
            // GameState::SizeChooser => { // TODO },
            GameState::LSystemChooser => {
                return self.viewer.file_chooser(self.window_width, self.window_height, "lsystem");
            }
            GameState::Help => {
                return self.viewer.help_screen(self.window_width, self.window_height);
            },
            GameState::GameOver => {
                return self.viewer.game_over_screen(&self.table, &self.player, self.window_width, self.window_height);
            },
            GameState::YouWin => {
                return self.viewer.win_screen(&self.player, self.window_width, self.window_height);
            },
            GameState::Playing | _ => {
                return self.viewer.draw_layout(&self.table, 
                                               &self.player, 
                                               self.player_control.max_speed, 
                                               self.player_control.fallover_threshold, 
                                               self.window_width, 
                                               self.window_height);
            },
        }
    }
}
