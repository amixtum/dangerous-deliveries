use console_engine::{ConsoleEngine, KeyCode};

use model::cell_table::CellTable;
use model::player::Player;
use view::game_viewer::GameViewer;
use controller::player_controller::PlayerController;

pub struct Game {
    table: CellTable,
    viewer: GameViewer,
    control: PlayerController,
    player: Player,

    engine: ConsoleEngine,
    window_width: u32,
    window_height: u32,

    redraw: bool,
    has_drawn: bool,
}

impl Game {
    pub fn new(window_width: u32, 
               window_height: u32, 
               target_fps: u32,
               table_width: u32, 
               table_height: u32, 
               conf_file: &str,
               lsystem_file: &str) -> Result<Self, String> {
        if let Ok(engine) = ConsoleEngine::init(window_width, window_height, target_fps) {
            return Ok(Game {
                table: CellTable::new(table_width as usize, table_height as usize, lsystem_file),
                viewer: GameViewer::new(32), // setting log length here, will specialize if needed
                control: PlayerController::new(conf_file),
                player: Player::new(0, 0, 0),
                engine,
                window_width,
                window_height,
                redraw: true,
                has_drawn: false,
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
        for keycode in self.control.get_keys() {
            if self.engine.is_key_pressed(*keycode) {
                let result = self.control.move_player(&self.table, &self.player, *keycode);
                self.viewer.add_message(&self.table, &self.player, &result.1);
                self.player = result.0;
                self.redraw = true;
            }
        }
    }

    pub fn draw(&mut self) {
        let screen = self.viewer.draw_layout(&self.table, &self.player, self.control.fallover_threshold, self.window_width, self.window_height);
        self.engine.print_screen(0, 0, &screen);
    }
}
