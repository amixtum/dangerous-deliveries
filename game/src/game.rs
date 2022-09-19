use console_engine::{Color, ConsoleEngine, KeyCode, KeyModifiers};
use controller::collision;
use model::map_gen;
use rand::Rng;
use util::heap::Heap;

use std::fs;

use util::{files, vec_ops};

use model::goal_table::GoalTable;
use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::{Player, PlayerType};
use model::player_event::PlayerEvent;
use model::state::GameState;

use view::view_manager::ViewManager;

use controller::ai_controller::AIController;
use controller::look_mode::LookMode;
use controller::player_controller::PlayerController;

pub struct Game {
    obs_table: ObstacleTable,
    goal_table: GoalTable,

    viewer: ViewManager,

    player_control: PlayerController,
    opponents: Vec<AIController>,
    lookmode: LookMode,

    player: Player,

    engine: ConsoleEngine,

    max_falls: u32,
    n_goals: u32,
    n_opponents: u32,
    sight_radius: u32,

    state: GameState,
    last_state: GameState,

    redraw: bool,
    first_draw: bool,
    gameover_done: bool,
    applied_automata: bool,

    current_lsystem: String,
}

impl Game {
    pub fn new(
        window_width: u32,
        window_height: u32,
        target_fps: u32,
        table_width: u32,
        table_height: u32,
        conf_file: &str,
        model_file: &str,
        lsystem_file: &str,
        table_file: &str,
    ) -> Result<Self, String> {
        if let Ok(engine) = ConsoleEngine::init(window_width, window_height, target_fps) {
            let mut g = Game {
                obs_table: ObstacleTable::new(table_width, table_height, lsystem_file, table_file),
                goal_table: GoalTable::new(),

                viewer: ViewManager::new(),

                player_control: PlayerController::new(model_file),
                opponents: Vec::new(),
                lookmode: LookMode::new(),

                player: Player::new(table_width as i32 / 2, table_height as i32 / 2),

                engine,

                max_falls: 4,
                n_goals: 4,
                n_opponents: 2,
                sight_radius: 8,

                state: GameState::MainMenu,
                last_state: GameState::MainMenu,

                redraw: true,
                first_draw: true,
                gameover_done: false,
                applied_automata: true,

                current_lsystem: String::new(),
            };

            if let Some(pair) = lsystem_file.rsplit_once('/') {
                g.current_lsystem.push_str(pair.1);
            }

            g.properties_from_file(conf_file);

            g.goal_table
                .regen_goals(g.obs_table.width(), g.obs_table.height(), g.n_goals);

            map_gen::voronoi_mapgen(&mut g.obs_table, &g.goal_table);

            g.clear_obstacles_at_goals();

            g.obs_table.set_obstacle(g.player.xy(), Obstacle::Platform);

            map_gen::tunnel_position(&mut g.obs_table, g.player.position);

            for _ in 0..g.n_opponents {
                g.add_opponent();
            }

            collision::update_blocked(&mut g.obs_table, &g.player, &g.opponents);

            return Ok(g);
        }
        Err(format!(
            "Could not create window of width {}, height {}, at target_fps {}",
            window_width, window_height, target_fps
        ))
    }
}

impl Game {
    // regen opponent
    fn add_opponent(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x = (self.obs_table.width() as i32 / 2)
            + rng.gen_range(
                -(self.obs_table.width() as i32) / 2 + 1..self.obs_table.width() as i32 / 2,
            )
            - 1;
        let mut y = (self.obs_table.height() as i32 / 2)
            + rng.gen_range(
                -(self.obs_table.height() as i32) / 2 + 1..self.obs_table.height() as i32 / 2 - 1,
            );

        while x == self.player.x() && y == self.player.y() {
            x = (self.obs_table.width() as i32 / 2)
                + rng.gen_range(
                    -(self.obs_table.width() as i32) / 2 + 1..self.obs_table.width() as i32 / 2 - 1,
                );
            y = (self.obs_table.height() as i32 / 2)
                + rng.gen_range(
                    -(self.obs_table.height() as i32) / 2 + 1
                        ..self.obs_table.height() as i32 / 2 - 1,
                );
        }

        self.opponents.push(AIController::new(x, y));
        self.obs_table.set_obstacle((x, y), Obstacle::Platform);

        map_gen::tunnel_position(&mut self.obs_table, (x, y));
    }

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
                } else if words[0] == "max_falls" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.max_falls = num;
                    }
                } else if words[0] == "opponents" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.n_opponents = num;
                    }
                } else if words[0] == "sight_radius" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        self.sight_radius = num;
                    }
                }
            }
        }
    }

    pub fn run(&mut self) -> bool {
        self.engine.wait_frame();

        if self
            .engine
            .is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL)
        {
            return false;
        }

        let done = self.handle_input();

        if self.first_draw {
            self.engine.clear_screen();
            self.print_screen();
            self.engine.draw();
            self.first_draw = false;
        }

        if let Some(resize) = self.engine.get_resize() {
            self.engine.resize(resize.0 as u32, resize.1 as u32);
            self.redraw = true;
        }

        if self.engine.is_key_pressed(KeyCode::Char('0')) {}

        if self.redraw {
            self.engine.clear_screen();
            self.print_screen();
            self.engine.draw();
        }

        done
    }

    pub fn handle_input(&mut self) -> bool {
        self.redraw = false;
        self.process()
    }

    fn ai_vec(&self) -> Vec<Player> {
        self.opponents.iter().map(|item| item.player).collect()
    }

    pub fn print_screen(&mut self) {
        let screen = self.viewer.get_screen(
            &self.state,
            &self.obs_table,
            &self.goal_table,
            &self.player,
            &self.ai_vec(),
            &self.player_control,
            self.max_falls,
            self.player_control.max_speed,
            self.player_control.fallover_threshold,
            self.engine.get_width(),
            self.engine.get_height(),
            &self.current_lsystem,
        );
        self.engine.print_screen(0, 0, &screen);
    }

    fn process(&mut self) -> bool {
        match self.state {
            GameState::MainMenu => {
                return self.process_main_menu();
            }
            GameState::SizeChooser => {
                return self.process_size_chooser();
            }
            GameState::LSystemChooser(_) => {
                return self.process_lsystem_chooser();
            }
            GameState::Help => {
                return self.process_help();
            }
            GameState::GameOver => {
                return self.process_gameover();
            }
            GameState::Playing => {
                return self.process_playing();
            }
            GameState::PostMove => {
                return self.process_post_move();
            }
            GameState::DeliveredPackage(x, y) => {
                return self.process_delivered(x, y);
            }
            GameState::LookMode => {
                return self.process_lookmode();
            }
            GameState::LookedAt(_) => {
                return self.process_looked_at();
            }
            GameState::Restart => {
                return self.process_restart();
            } /*
              _  => {
                  return false;
              },*/
        }
    }

    fn process_main_menu(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Char('q')) {
            return false;
        } else if self.engine.is_key_pressed(KeyCode::Char('0')) {
            self.set_state(GameState::Help);
        } else if self.engine.is_key_pressed(KeyCode::Char('1'))
            || self.engine.is_key_pressed(KeyCode::Esc)
        {
            self.set_state(match self.last_state {
                GameState::LookMode => GameState::LookMode,
                _ => GameState::Playing,
            });
        } /* else if self.engine.is_key_pressed(KeyCode::Char('2')) {

              self.set_state(GameState::SizeChooser);
          }*/

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
        } else if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }

        return true;
    }

    fn process_restart(&mut self) -> bool {
        self.reset_game();
        self.set_state(GameState::Playing);
        self.applied_automata = true;
        self.viewer.main_view.clear_log();
        return true;
    }

    fn process_playing(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        } else if self.engine.is_key_pressed(KeyCode::Char(';')) {
            self.viewer
                .main_view
                .add_string(String::from("Look Where?"), Color::Yellow);
            self.set_state(GameState::LookMode);
        } else if self.engine.is_key_pressed(KeyCode::Enter) {
            self.set_state(GameState::Restart);
        }
        /*else if self.engine.is_key_pressed(KeyCode::Char('g')) {
            if !self.applied_automata {
                self.obs_table.apply_automata();
                let w = self.obs_table.width() / 4;
                let h = self.obs_table.height() / 4;
                //apply_voronoi(&mut self.obs_table, w as usize * h as usize);
                self.obs_table.set_obstacle(
                    self.player.xy(),
                    Obstacle::Platform(0),
                );
                self.clear_obstacles_at_goals();
                self.applied_automata = true;
                self.redraw = true;
            }
        }*/
        else {
            let keysv = self.player_control.get_keys();
            for keycode in keysv {
                if self.engine.is_key_pressed(keycode) {
                    // compute turn order
                    let mut heap = Heap::new();

                    // insert the human player
                    heap.insert(
                        (100.0 / vec_ops::magnitude(self.player.speed)) as u32,
                        (999, PlayerType::Human),
                    );

                    // insert the ai opponents
                    for index in 0..self.opponents.len() {
                        heap.insert(
                            (100.0 / vec_ops::magnitude(self.opponents[index].player.speed)) as u32,
                            (index, PlayerType::AI),
                        );
                    }

                    while !heap.empty() {
                        let goes_next = heap.extract_min();
                        match goes_next.1 {
                            PlayerType::Human => {
                                self.process_move_human(keycode);
                            }
                            PlayerType::AI => {
                                self.process_ai(goes_next.0);
                            }
                        }

                        collision::update_blocked(
                            &mut self.obs_table,
                            &self.player,
                            &self.opponents,
                        );
                    }
                    break;
                }
            }
        }

        return true;
    }

    fn process_ai(&mut self, index: usize) {
        let p = &self.opponents[index].player;
        if vec_ops::magnitude((
            (p.x() - self.player.x()) as f32,
            (p.y() - self.player.y()) as f32,
        )) <= self.sight_radius as f32
        {
            self.opponents[index].choose_goal(&self.player);
        } else {
            self.opponents[index].goal = (-1, -1);
        }
        self.opponents[index].move_player(&self.obs_table, &self.player_control);

        if self.goal_table.count() <= 0 {
            self.set_state(GameState::GameOver);
            self.player.recent_event = PlayerEvent::GameOver(self.player.time.round() as i32);
        } else if let PlayerEvent::GameOver(_) = self.opponents[index].player.recent_event {
            self.opponents[index].player =
                PlayerController::reset_ai_continue(&self.obs_table, &self.opponents[index].player);
        } else if self.opponents[index].player.n_falls >= self.max_falls as i32 {
            self.opponents[index].player =
                PlayerController::reset_ai_continue(&self.obs_table, &self.opponents[index].player);
        }

        //self.opponents[index].choose_goal(&self.goal_table);

        self.redraw = true;
    }

    fn process_move_human(&mut self, keycode: KeyCode) {
        // move player according to the key pressed
        let result = self
            .player_control
            .move_player(&self.obs_table, &self.player, keycode);
        self.player = result;

        // check if we reached a goal
        if self.goal_table.remove_goal_if_reached(self.player.xy()) {
            self.set_state(GameState::DeliveredPackage(
                self.player.x(),
                self.player.y(),
            ));
        }

        // check if the player has reached all the goals
        if self.goal_table.count() <= 0 {
            self.set_state(GameState::GameOver);
            self.player.recent_event = PlayerEvent::GameOver(self.player.time.round() as i32);
        }
        // check if move player returned a player with a GameOver event
        else if let PlayerEvent::GameOver(_) = self.player.recent_event {
            self.reset_player_continue();
        }
        // check if the player's hp has reached 0
        else if self.player.n_falls >= self.max_falls as i32 {
            self.reset_player_continue();
        }
        // otherwise go to the state where we update the message log
        // after computing the result of the turn
        else {
            self.set_state(match self.state {
                GameState::DeliveredPackage(x, y) => GameState::DeliveredPackage(x, y),
                _ => GameState::PostMove,
            });
        }

        match self.player.recent_event {
            PlayerEvent::OnRail | PlayerEvent::OffRail => {
                self.applied_automata = true;
            }
            PlayerEvent::Wait => {}
            _ => {
                self.applied_automata = false;
            }
        }
    }

    // dummy state for the purposes of updating the view after process_move
    fn process_post_move(&mut self) -> bool {
        self.set_state(GameState::Playing);
        self.viewer
            .main_view
            .add_message(&self.obs_table, &self.player, &self.player.recent_event);
        return true;
    }

    fn process_lookmode(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        }

        for keycode in self.lookmode.get_keys() {
            if self.engine.is_key_pressed(*keycode) {
                let result =
                    self.lookmode
                        .describe_direction(&self.obs_table, &self.player, *keycode);
                self.set_state(GameState::LookedAt(result));
                break;
            }
        }

        return true;
    }

    fn process_looked_at(&mut self) -> bool {
        if let GameState::LookedAt(s) = &self.state {
            self.viewer
                .main_view
                .add_string(String::from(s), Color::Green);
        }
        self.set_state(GameState::Playing);
        return true;
    }

    fn process_delivered(&mut self, _x: i32, _y: i32) -> bool {
        self.player.n_delivered += 1;

        self.viewer
            .main_view
            .add_string(String::from("Delivered"), Color::Blue);

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
            let filenames =
                files::get_config_filenames(&files::get_file_chooser_string(size_index as u32));
            let mut index = 0;
            while index < filenames.len() {
                if let Some(c) = index.to_string().chars().nth(0) {
                    if self.engine.is_key_pressed(KeyCode::Char(c)) {
                        let mut lsystems =
                            files::get_lsystems(&files::get_file_chooser_string(size_index as u32));
                        self.current_lsystem.clear();
                        self.current_lsystem.push_str(&filenames[index]);

                        let lsystem = lsystems.remove(index);
                        self.obs_table.set_lsystem(lsystem);

                        self.goal_table.regen_goals(
                            self.obs_table.width(),
                            self.obs_table.height(),
                            self.n_goals,
                        );
                        self.clear_obstacles_at_goals();
                        self.player =
                            PlayerController::reset_player_gameover(&self.obs_table, &self.player);
                        self.obs_table
                            .set_obstacle(self.player.xy(), Obstacle::Platform);
                        self.opponents.clear();
                        for _ in 0..self.n_opponents {
                            self.add_opponent();
                        }
                        self.set_state(GameState::Playing);
                        self.applied_automata = true;
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
        self.goal_table.regen_goals(
            self.obs_table.width(),
            self.obs_table.height(),
            self.n_goals,
        );

        map_gen::voronoi_mapgen(&mut self.obs_table, &self.goal_table);

        self.clear_obstacles_at_goals();

        self.player = PlayerController::reset_player_gameover(&self.obs_table, &self.player);
        self.obs_table
            .set_obstacle(self.player.xy(), Obstacle::Platform);

        self.opponents.clear();
        for _ in 0..self.n_opponents {
            self.add_opponent();
        }

        map_gen::tunnel_position(&mut self.obs_table, self.player.position);
    }

    fn reset_player_continue(&mut self) {
        self.player = PlayerController::reset_player_continue(&self.obs_table, &self.player);
        self.obs_table
            .set_obstacle(self.player.xy(), Obstacle::Platform);
        self.redraw = true;
    }

    fn clear_obstacles_at_goals(&mut self) {
        for goal in self.goal_table.goals() {
            self.obs_table.set_obstacle(*goal, Obstacle::Platform);
        }
    }

    fn process_size_chooser(&mut self) -> bool {
        if self.engine.is_key_pressed(KeyCode::Esc) {
            self.set_state(GameState::MainMenu);
        } else if self.engine.is_key_pressed(KeyCode::Char('0')) {
            self.obs_table.resize(40, 20);
            self.goal_table.regen_goals(
                self.obs_table.width(),
                self.obs_table.height(),
                self.n_goals,
            );
            self.set_state(GameState::LSystemChooser(0));
        } else if self.engine.is_key_pressed(KeyCode::Char('1')) {
            self.obs_table.resize(80, 40);
            self.goal_table.regen_goals(
                self.obs_table.width(),
                self.obs_table.height(),
                self.n_goals,
            );
            self.set_state(GameState::LSystemChooser(1));
        }

        return true;
    }
}
