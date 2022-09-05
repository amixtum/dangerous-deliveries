use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};

use std::fs;

use model::state::GameState;

//use util::files;

use model::obstacle_table::ObstacleTable;
use model::goal_table::GoalTable;
use model::player::Player;
use model::player_event::PlayerEvent;

use view::view_manager::ViewManager;

use controller::player_controller::PlayerController;
use controller::look_mode::LookMode;

pub struct Game {
    obs_table: ObstacleTable,
    goal_table: GoalTable,

    viewer: ViewManager,

    player_control: PlayerController,
    lookmode: LookMode,

    player: Player,

    engine: ConsoleEngine,
    window_width: u32,
    window_height: u32,

    max_falls: u32,
    n_goals: u32,

    state: GameState,

    redraw: bool,
    first_draw: bool,
    gameover_done: bool,

}

impl Game {
    pub fn new(window_width: u32, 
               window_height: u32, 
               target_fps: u32,
               table_width: u32, 
               table_height: u32, 
               conf_file: &str,
               model_file: &str,
               lsystem_file: &str,
               table_file: &str,) -> Result<Self, String> {
        if let Ok(engine) = ConsoleEngine::init(window_width, window_height, target_fps) {
            let mut g = Game {
                obs_table: ObstacleTable::new(table_width, table_height, lsystem_file, table_file),
                goal_table: GoalTable::new(),

                viewer: ViewManager::new(),

                player_control: PlayerController::new(model_file),
                lookmode: LookMode::new(),

                player: Player::new(table_width as i32 / 2, table_height as i32 / 2, 0),

                engine,
                window_width,
                window_height,

                max_falls: 4,
                n_goals: 4,

                state: GameState::MainMenu,

                redraw: true,
                first_draw: true,
                gameover_done: false,

            };

            g.properties_from_file(conf_file);

            g.goal_table.regen_goals(g.obs_table.width(), g.obs_table.height(), g.n_goals);

            return Ok(g);
        }
        Err(format!("Could not create window of width {}, height {}, at target_fps {}", window_width, window_height, target_fps))
    }
}

impl Game {
    pub fn properties_from_file(&mut self, path: &str) {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if let Some(c) = line.chars().nth(0) {
                    if c == '#' {
                        continue;
                    }
                } else {
                    continue;
                }

                let words: Vec<&str> = line.split_ascii_whitespace().collect(); 
                if words[0] == "n_goals" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.n_goals = num;
                    }
                }
                else if words[0] == "max_falls" {
                     if let Ok(num) = words[1].parse::<u32>() {
                        self.max_falls = num;
                    }                   
                }
            }
        }
    }

    pub fn run(&mut self) -> bool {
        self.engine.wait_frame();

        if self.engine.is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            return false;
        }

        let done = self.handle_input(); 

        if self.first_draw {
            self.engine.clear_screen();
            self.draw();
            self.engine.draw();
            self.first_draw = false;
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
        let screen = self.viewer.get_screen(
            &self.state,
            &self.obs_table,
            &self.goal_table,
            &self.player,
            self.n_goals,
            self.max_falls,
            self.player_control.max_speed,
            self.player_control.fallover_threshold,
            self.window_width,
            self.window_height,
        );
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
            GameState::PostMove => {
                return self.process_post_move();
            },
            GameState::DeliveredPackage => {
                return self.process_delivered();
            },
            GameState::LookMode => {
                return self.process_lookmode();
            },
            GameState::LookedAt(_) => {
                return self.process_looked_at();
            }
            GameState::Restart => {
                return self.process_restart();
            },
            /*
            _  => {
                return false;
            },*/
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
            self.obs_table.regen_table();
            self.goal_table.regen_goals(self.obs_table.width(), self.obs_table.height(), self.n_goals);
            self.player = PlayerController::reset_player(&self.obs_table, &self.player);
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

    fn process_restart(&mut self) -> bool {
        self.obs_table.regen_table();
        self.goal_table.regen_goals(self.obs_table.width(), self.obs_table.height(), self.n_goals);
        self.player = PlayerController::reset_player(&self.obs_table, &self.player);
        self.redraw = true;
        self.state = GameState::Playing;
        return true;
    }

    fn process_playing(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::MainMenu;
            self.redraw = true;
        }
        else if self.engine.is_key_pressed(KeyCode::Char(';')) {
            self.state = GameState::LookMode;
            self.redraw = true;
        }
        else if self.engine.is_key_pressed(KeyCode::Enter) {
            self.state = GameState::Restart;
            self.redraw = true;
        }
        else {
            for keycode in self.player_control.get_keys() {
                if self.engine.is_key_pressed(*keycode) {
                    let result = self.player_control.move_player(&self.obs_table, &self.player, *keycode);

                    self.player = result;

                    if self.goal_table.remove_goal_if_reached(self.player.xy()) {
                        self.state = GameState::DeliveredPackage;
                        self.redraw = true;
                    }

                    if self.goal_table.count() <= 0 {
                        self.state = GameState::YouWin;
                        self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                    }
                    else if let PlayerEvent::GameOver(_) = self.player.recent_event {
                        self.state = GameState::GameOver;
                    }
                    else if self.player.n_falls >= self.max_falls as i32 {
                        self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                        self.state = GameState::GameOver;
                    }
                    else {
                        self.state = match self.state {
                            GameState::DeliveredPackage => GameState::DeliveredPackage,
                            _ => GameState::PostMove,
                        };
                    }

                    self.redraw = true;

                    break;
                }
            }
        }

        return true;
    }

    fn process_post_move(&mut self) -> bool {
        self.state = GameState::Playing;
        self.redraw = true;
        return true;
    }


    fn process_lookmode(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.state = GameState::MainMenu;
            self.redraw = true;
        }

        for keycode in self.lookmode.get_keys()  {
             if self.engine.is_key_pressed(*keycode) {
                let result = self.lookmode.describe_direction(&self.obs_table, &self.player, *keycode);
                self.state = GameState::LookedAt(result);
                self.redraw = true;
                break;
             }
        }

        return true;
    }

    fn process_looked_at(&mut self) -> bool {
        self.state = GameState::Playing;
        self.redraw = true;
        return true;
    }

    fn process_delivered(&mut self) -> bool {
        self.state = GameState::Playing;
        self.redraw = true;
        return true;
    }

    /*
    fn process_lsystem_chooser(&mut self) -> bool {
        let mut lsystems = files::get_lsystems();
        let mut index = 0;
        while index < lsystems.len() {
            if let Some(c) = index.to_string().chars().nth(0) {
                if self.engine.is_key_pressed(KeyCode::Char(c)) {
                    let lsystem = lsystems.remove(index);
                    self.obs_table.set_lsystem(lsystem);
                    self.state = GameState::MainMenu;
                    break;
                }
            }
            index += 1;
        }

        return true;
    }
    */
}
