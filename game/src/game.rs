use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};


use std::fs;

use util::files;

use model::obstacle::Obstacle;
use model::state::GameState;
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
    last_state: GameState,

    redraw: bool,
    first_draw: bool,
    gameover_done: bool,

    current_lsystem: String,
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
                last_state: GameState::MainMenu,

                redraw: true,
                first_draw: true,
                gameover_done: false,

                current_lsystem: String::new(), 
            };

            if let Some(pair) = lsystem_file.rsplit_once('/') {
                g.current_lsystem.push_str(pair.1);
            }

            g.properties_from_file(conf_file);

            g.goal_table.regen_goals(g.obs_table.width(), g.obs_table.height(), g.n_goals);
            g.clear_obstacles_at_goals();

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
            //self.engine.clear_screen();
            self.print_screen();
            self.engine.draw();
            self.first_draw = false;
        }

        if self.redraw {
            //self.engine.clear_screen();
            self.print_screen();
            self.engine.draw();
        }

        done
    }

    pub fn handle_input(&mut self) -> bool {
        self.redraw = false;
        self.process()
    }

    pub fn print_screen(&mut self) {
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
            &self.current_lsystem,
        );
        self.engine.print_screen(0, 0, &screen);
    }

    fn process(&mut self) -> bool {
        match self.state {
            GameState::MainMenu => {
                return self.process_main_menu();
            },
            GameState::SizeChooser => {
                return self.process_size_chooser();
            },
            GameState::LSystemChooser(_) => {
                return self.process_lsystem_chooser();
            }
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
            self.set_state(GameState::Help);
        }
        else if self.engine.is_key_pressed(KeyCode::Char('1')) || self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(match self.last_state {
                GameState::LookMode => GameState::LookMode,
                _ => GameState::Playing, 
            });
        }
        else if self.engine.is_key_pressed(KeyCode::Char('2')) {
            self.set_state(GameState::SizeChooser);
        }

        return true;
    }

    fn process_help(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }

        return true;
    }

    fn process_gameover(&mut self) -> bool {
        if !self.gameover_done {
            self.reset_game();
            self.gameover_done = true;
        }

        if self.engine.is_key_pressed(KeyCode::Char('r')) {
            self.set_state(GameState::Playing);
            self.gameover_done = false;
        }
        else if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }

        return true;
    }

    fn process_restart(&mut self) -> bool {
        self.reset_game();
        self.set_state(GameState::Playing);
        return true;
    }

    fn process_playing(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }
        else if self.engine.is_key_pressed(KeyCode::Char(';')) {
            self.set_state(GameState::LookMode);
        }
        else if self.engine.is_key_pressed(KeyCode::Enter) {
            self.set_state(GameState::Restart);
        }
        else {
            for keycode in self.player_control.get_keys() {
                if self.engine.is_key_pressed(*keycode) {
                    // move player according to the key pressed
                    let result = self.player_control.move_player(&self.obs_table, &self.player, *keycode);
                    self.player = result;

                    // check if we reached a goal
                    if self.goal_table.remove_goal_if_reached(self.player.xy()) {
                        self.set_state(GameState::DeliveredPackage);
                    }

                    // check if the player has reached all the goals
                    if self.goal_table.count() <= 0 {
                        self.set_state(GameState::YouWin);
                        self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                    }

                    // check if move player returned a player with a GameOver event
                    else if let PlayerEvent::GameOver(_) = self.player.recent_event {
                        self.set_state(GameState::GameOver);
                    }

                    // check if the player's hp has reached 0
                    else if self.player.n_falls >= self.max_falls as i32 {
                        self.player.recent_event = PlayerEvent::GameOver(self.player.time as i32);
                        self.set_state(GameState::GameOver);
                    }

                    // otherwise go to the state where we update the message log
                    // after computing the result of the turn
                    else {
                        self.set_state(match self.state {
                            GameState::DeliveredPackage => GameState::DeliveredPackage,
                            _ => GameState::PostMove,
                        });
                    }

                    break;
                }
            }
        }

        return true;
    }

    // dummy state for the purposes of updating the view after process_move
    fn process_post_move(&mut self) -> bool {
        self.set_state(GameState::Playing);
        return true;
    }


    fn process_lookmode(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }

        for keycode in self.lookmode.get_keys()  {
             if self.engine.is_key_pressed(*keycode) {
                let result = self.lookmode.describe_direction(&self.obs_table, &self.player, *keycode);
                self.set_state(GameState::LookedAt(result));
                break;
             }
        }

        return true;
    }

    fn process_looked_at(&mut self) -> bool {
        self.set_state(GameState::Playing);
        return true;
    }

    fn process_delivered(&mut self) -> bool {
        self.set_state(GameState::Playing);
        return true;
    }

    fn set_state(&mut self, state: GameState) {
        self.last_state = GameState::clone(&self.state);
        self.state = state;
        self.redraw = true;
    }

    fn process_lsystem_chooser(&mut self) -> bool {
        if let GameState::LSystemChooser(size_index) = self.state {
            let mut lsystems = files::get_lsystems(&files::get_file_chooser_string(size_index as u32));
            let filenames = files::get_config_filenames(&files::get_file_chooser_string(size_index as u32));
            let mut index = 0;
            while index < lsystems.len() {
                if let Some(c) = index.to_string().chars().nth(0) {
                    if self.engine.is_key_pressed(KeyCode::Char(c)) {
                        self.current_lsystem.clear();
                        self.current_lsystem.push_str(&filenames[index]);
                        let lsystem = lsystems.remove(index);
                        self.obs_table.set_lsystem(lsystem);
                        self.goal_table.regen_goals(self.obs_table.width(), self.obs_table.height(), self.n_goals);
                        self.clear_obstacles_at_goals();
                        self.player = PlayerController::reset_player(&self.obs_table, &self.player);
                        self.set_state(GameState::Playing);
                        break;
                    }
                }
                index += 1;
            }           
        }

        return true;
    }

    fn reset_game(&mut self) {
        self.obs_table.regen_table();
        self.goal_table.regen_goals(self.obs_table.width(), self.obs_table.height(), self.n_goals);
        self.clear_obstacles_at_goals();
        self.player = PlayerController::reset_player(&self.obs_table, &self.player);
    }

    fn clear_obstacles_at_goals(&mut self) {
        for goal in self.goal_table.goals() {
            self.obs_table.set_obstacle(*goal, Obstacle::Platform(0));
        }
    }

    fn process_size_chooser(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }
        else if self.engine.is_key_pressed(KeyCode::Char('0')) {
            self.obs_table.resize(40, 20);
            self.goal_table.regen_goals(self.obs_table.width(), self.obs_table.height(), self.n_goals);
            self.set_state(GameState::LSystemChooser(0));
        }
        else if self.engine.is_key_pressed(KeyCode::Char('1')) {
            self.obs_table.resize(80, 40);
            self.goal_table.regen_goals(self.obs_table.width(), self.obs_table.height(), self.n_goals);
            self.set_state(GameState::LSystemChooser(1));
        }

        return true;
    }
}
