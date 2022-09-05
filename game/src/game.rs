use console_engine::{ConsoleEngine, screen::Screen, KeyCode, KeyModifiers};

use super::state::GameState;
use util::files;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;
use view::game_viewer::GameViewer;
use controller::player_controller::PlayerController;
use controller::look_mode::LookMode;

pub struct Game {
    table: ObstacleTable,
    viewer: GameViewer,
    player_control: PlayerController,
    lookmode: LookMode,
    player: Player,

    engine: ConsoleEngine,
    window_width: u32,
    window_height: u32,

    redraw: bool,
    has_drawn: bool,
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
               table_file: &str,) -> Result<Self, String> {
        if let Ok(engine) = ConsoleEngine::init(window_width, window_height, target_fps) {
            return Ok(Game {
                table: ObstacleTable::new(table_width, table_height, lsystem_file, table_file),
                viewer: GameViewer::new(64), // setting log length here, will specialize if needed
                player_control: PlayerController::new(conf_file),
                lookmode: LookMode::new(),
                player: Player::new(table_width as i32 / 2, table_height as i32 / 2, 0),

                engine,
                window_width,
                window_height,

                redraw: true,
                has_drawn: false,

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

        if self.engine.is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            return false;
        }

        let done = self.handle_input(); 

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

        done
    }

    pub fn handle_input(&mut self) -> bool {
        self.redraw = false;
        self.process()
    }

    pub fn draw(&mut self) {
        let screen = self.get_screen();
        self.engine.print_screen(0, 0, &screen);
    }

    fn process(&mut self) -> bool {
        match self.state {
            GameState::MainMenu => {
                return self.process_main_menu();
            },
            GameState::Help => {
                return self.process_help();
            },
            GameState::GameOver | GameState::YouWin => {
                return self.process_gameover();
            },
            GameState::Playing => {
                return self.process_playing();
            },
            GameState::LookMode => {
                return self.process_lookmode();
            },
            GameState::LSystemChooser => {
                return self.process_lsystem_chooser();
            },
            _  => {
                return false;
            },
        }
    }

    fn get_screen(&self) -> Screen {
        match self.state {
            GameState::MainMenu => {
                return self.viewer.draw_main_menu(self.window_width, self.window_height);
            },
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

    fn process_main_menu(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Char('q')) {
            return false;
        }

        else if self.engine.is_key_pressed(KeyCode::Char('0')) {
            self.state = GameState::Help;
            self.redraw = true;
        }
        else if self.engine.is_key_pressed(KeyCode::Char('1')) || self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::Playing;
            self.redraw = true;
        }

        return true;
    }

    fn process_help(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::MainMenu;
            self.redraw = true;
        }

        return true;
    }

    fn process_gameover(&mut self) -> bool {
        if !self.gameover_done {
            self.table.regen_table();
            self.player = PlayerController::reset_player(&self.table, &self.player);
            self.viewer.clear_log();
            self.gameover_done = true;
        }

        if self.engine.is_key_pressed(KeyCode::Char('r')) {
            self.state = GameState::Playing;
            self.gameover_done = false;
            self.redraw = true;
        }
        else if self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::MainMenu;
            self.redraw = true;
        }

        return true;
    }

    fn process_playing(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::MainMenu;
            self.redraw = true;
        }
        else if self.engine.is_key_pressed(KeyCode::Char(';')) {
            self.state = GameState::LookMode;
            self.viewer.add_string(String::from("Look where?"));
            self.redraw = true;
        }
        else if self.engine.is_key_pressed(KeyCode::Enter) {
            self.table.regen_table();
            self.player = PlayerController::reset_player(&self.table, &self.player);
            self.viewer.clear_log();
            self.redraw = true;
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
                        self.state = GameState::GameOver;
                    } else if let PlayerEvent::FallOver = self.player.recent_event {
                        self.table.inc_fallover();
                        if self.table.check_falls() {
                            self.state = GameState::GameOver;
                            self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                        }
                    }

                    if self.table.get_goals().len() <= 0 {
                        self.state = GameState::YouWin;
                        self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                    }

                    self.redraw = true;

                    break;
                }
            }
        }

        return true;
    }

    fn process_lookmode(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::MainMenu;
        }

        for keycode in self.lookmode.get_keys()  {
             if self.engine.is_key_pressed(*keycode) {
                let result = self.lookmode.describe_direction(&self.table, &self.player, *keycode);
                self.viewer.add_string(result);
                self.redraw = true;
                self.state = GameState::Playing;
                break;
             }
        }

        return true;
    }

    fn process_lsystem_chooser(&mut self) -> bool {
        let mut lsystems = files::get_lsystems();
        let mut index = 0;
        while index < lsystems.len() {
            if let Some(c) = index.to_string().chars().nth(0) {
                if self.engine.is_key_pressed(KeyCode::Char(c)) {
                    let lsystem = lsystems.remove(index);
                    self.table.set_lsystem(lsystem);
                    self.state = GameState::MainMenu;
                    break;
                }
            }
            index += 1;
        }

        return true;
    }
}
