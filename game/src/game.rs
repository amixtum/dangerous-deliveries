use console_engine::{ConsoleEngine, KeyCode};

use model::cell_table::CellTable;
use model::player::Player;
use model::player_event::PlayerEvent;
use view::game_viewer::GameViewer;
use controller::player_controller::PlayerController;
use controller::look_mode::LookMode;

pub struct Game {
    table: CellTable,
    viewer: GameViewer,
    control: PlayerController,
    lookmode: LookMode,
    player: Player,

    engine: ConsoleEngine,
    window_width: u32,
    window_height: u32,

    redraw: bool,
    has_drawn: bool,
    lookmode_on: bool,
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
                table: CellTable::new(table_width as usize, table_height as usize, lsystem_file, turtle_file),
                viewer: GameViewer::new(64), // setting log length here, will specialize if needed
                control: PlayerController::new(conf_file),
                lookmode: LookMode::new(),
                player: Player::new(table_width as i32 / 2, table_height as i32 / 2, 0),
                engine,
                window_width,
                window_height,
                redraw: true,
                has_drawn: false,
                lookmode_on: false,
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

        if self.engine.is_key_pressed(KeyCode::Char('q')) {
            return false;
        }

        true
    }

    pub fn handle_input(&mut self) {
        self.redraw = false;
        if self.engine.is_key_pressed(KeyCode::Char(';')) {
            self.viewer.add_string(String::from("Look where?"));
            self.redraw = true;
            self.lookmode_on = true; 
        }
        else if self.engine.is_key_pressed(KeyCode::Char('r')) {
            self.table.regen_table();
            self.player = self.table.reset_player(&self.player);
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
            for keycode in self.control.get_keys() {
                if self.engine.is_key_pressed(*keycode) {
                    let result = self.control.move_player(&self.table, &self.player, *keycode);

                    self.viewer.add_message(&self.table, &result.0, &result.1);

                    self.player = result.0;

                    if self.table.remove_goal_if_reached(&self.player) {
                        self.viewer.add_string(String::from("Delivered"));
                    }

                    if let PlayerEvent::GameOver = self.player.recent_event {
                        self.table.regen_table();
                    } else if let PlayerEvent::FallOver = self.player.recent_event {
                        self.table.inc_fallover();
                        if self.table.check_falls() {
                            self.viewer.add_string(String::from("Game Over"));
                            self.player = self.table.reset_player(&self.player);
                            self.table.regen_table();
                        }
                    }

                    if self.table.get_goals().len() <= 0 {
                        self.viewer.add_string(String::from("You Win!"));
                        self.table.regen_table();
                    }


                    self.redraw = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        let screen = self.viewer.draw_layout(&self.table, &self.player, self.control.max_speed, self.control.fallover_threshold, self.window_width, self.window_height);
        self.engine.print_screen(0, 0, &screen);
    }
}
