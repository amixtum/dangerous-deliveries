use controller::collision;
use model::map_gen;
use rltk::{GameState, VirtualKeyCode, RGB, RandomNumberGenerator};
use util::heap::Heap;

use util::{vec_ops};

use model::goal_table::GoalTable;
use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::{Player, PlayerType};
use model::player_event::PlayerEvent;
use model::state::ProcState;

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

    max_falls: u32,
    n_goals: u32,
    n_opponents: u32,
    sight_radius: u32,

    state: ProcState,
    last_state: ProcState,

    redraw: bool,
    first_draw: bool,
    gameover_done: bool,
    applied_automata: bool,
}

impl Game {
    pub fn new(
        table_width: u32,
        table_height: u32,
    ) -> Self {
        
            let mut g = Game {
                obs_table: ObstacleTable::new(table_width, table_height),
                goal_table: GoalTable::new(),

                viewer: ViewManager::new(),

                player_control: PlayerController::new(),
                opponents: Vec::new(),
                lookmode: LookMode::new(),

                player: Player::new(table_width as i32 / 2, table_height as i32 / 2),

                max_falls: 4,
                n_goals: 4,
                n_opponents: 2,
                sight_radius: 8,

                state: ProcState::MainMenu,
                last_state: ProcState::MainMenu,

                redraw: true,
                first_draw: true,
                gameover_done: false,
                applied_automata: true,
            };

            Game::init(&mut g);

            return g;
    }
}


impl GameState for Game {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::C => {
                    if ctx.control {
                        std::process::exit(0);
                    }
                }
                _ => {}
            }
        }

        let playing = self.handle_input(ctx);

        if !playing {
            std::process::exit(0);
        }

        if self.first_draw {
            ctx.cls();
            self.print_screen(ctx);
            self.first_draw = false;
        }

        if self.redraw {
            ctx.cls();
            self.print_screen(ctx);
        }


    }       
}

impl Game {
    fn init(g: &mut Game) {
        g.goal_table
            .regen_goals(g.obs_table.width(), g.obs_table.height(), g.n_goals);

        map_gen::voronoi_mapgen(&mut g.obs_table, &g.goal_table);

        g.clear_obstacles_at_goals();

        g.obs_table.set_obstacle(g.player.xy(), Obstacle::Platform);

        map_gen::tunnel_position(&mut g.obs_table, g.player.position);

        for _ in 0..g.n_opponents {
            g.add_opponent();
        }

        g.properties_from_file();

        collision::update_blocked(&mut g.obs_table, &g.player, &g.opponents);
    }
    // regen opponent
    fn add_opponent(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let mut x = (self.obs_table.width() as i32 / 2)
            + rng.range(
                -(self.obs_table.width() as i32) / 2 + 1, self.obs_table.width() as i32 / 2,
            )
            - 1;
        let mut y = (self.obs_table.height() as i32 / 2)
            + rng.range(
                -(self.obs_table.height() as i32) / 2 + 1, self.obs_table.height() as i32 / 2 - 1,
            );

        while x == self.player.x() && y == self.player.y() {
            x = (self.obs_table.width() as i32 / 2)
                + rng.range(
                    -(self.obs_table.width() as i32) / 2 + 1, self.obs_table.width() as i32 / 2 - 1,
                );
            y = (self.obs_table.height() as i32 / 2)
                + rng.range(
                    -(self.obs_table.height() as i32) / 2 + 1
                        , self.obs_table.height() as i32 / 2 - 1,
                );
        }

        self.opponents.push(AIController::new(x, y));
        self.obs_table.set_obstacle((x, y), Obstacle::Platform);

        map_gen::tunnel_position(&mut self.obs_table, (x, y));
    }

    pub fn properties_from_file(&mut self) {
// Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        let raw_data = rltk::embedding::EMBED
            .lock()
            .get_resource("/home/ganiparrott/src/projects/rust_book/roguelike/raws/game.txt".to_string())
            .unwrap();
        let raw_string = std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");
        for line in raw_string.lines() {
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



    pub fn handle_input(&mut self, ctx: &mut rltk::Rltk) -> bool {
        self.redraw = false;
        self.process(ctx)
    }

    fn ai_vec(&self) -> Vec<Player> {
        self.opponents.iter().map(|item| item.player).collect()
    }

    pub fn print_screen(&mut self, ctx: &mut rltk::Rltk) {
        self.viewer.get_screen(
            ctx,
            &self.state,
            &self.obs_table,
            &self.goal_table,
            &self.player,
            &self.ai_vec(),
            &self.player_control,
            self.max_falls,
            self.player_control.max_speed,
            self.player_control.fallover_threshold,
            ctx.get_char_size().0,
            ctx.get_char_size().1,
        );
    }

    fn process(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match self.state {
            ProcState::MainMenu => {
                return self.process_main_menu(ctx);
            }
            ProcState::Help => {
                return self.process_help(ctx);
            }
            ProcState::GameOver => {
                return self.process_gameover(ctx);
            }
            ProcState::Playing => {
                return self.process_playing(ctx);
            }
            ProcState::PostMove => {
                return self.process_post_move();
            }
            ProcState::DeliveredPackage(x, y) => {
                return self.process_delivered(x, y);
            }
            ProcState::LookMode => {
                return self.process_lookmode(ctx);
            }
            ProcState::LookedAt(_) => {
                return self.process_looked_at();
            }
            ProcState::Restart => {
                return self.process_restart();
            } /*
              _  => {
                  return false;
              },*/
        }
    }

    fn process_main_menu(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match ctx.key {
            None => {},
            Some(key) => match key {
                VirtualKeyCode::Q => {  
                    return false;
                },
                VirtualKeyCode::Key0 => {
                    self.set_state(ProcState::Help);
                }
                VirtualKeyCode::Escape |
                VirtualKeyCode::Key1 => {
                    self.set_state(match self.last_state {
                        ProcState::LookMode => ProcState::LookMode,
                        _ => ProcState::Playing,
                    });
                }
                _ => {},
            }
        }

        return true;
    }

    fn process_help(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match ctx.key {
            None => {},
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    self.set_state(ProcState::MainMenu);
                },
                _ => {},
            }
        }

        return true;
    }

    fn process_gameover(&mut self, ctx: &mut rltk::Rltk) -> bool {
        if !self.gameover_done {
            self.reset_game();
            self.gameover_done = true;
        }

        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::R => {
                    self.set_state(ProcState::Playing);
                    self.gameover_done = false;
                }
                VirtualKeyCode::Escape => {
                    self.set_state(ProcState::MainMenu);
                }
                _ => {},
            }
        }

        return true;
    }

    fn process_restart(&mut self) -> bool {
        self.reset_game();
        self.set_state(ProcState::Playing);
        self.applied_automata = true;
        self.viewer.main_view.clear_log();
        return true;
    }

    fn process_playing(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    self.set_state(ProcState::MainMenu);
                }
                VirtualKeyCode::Semicolon => {
                    self.viewer
                        .main_view
                        .add_string(String::from("Look Where?"), RGB::named(rltk::YELLOW));
                    self.set_state(ProcState::LookMode);
                }
                VirtualKeyCode::Return => {
                    self.set_state(ProcState::Restart);
                }
                _ => {
                    let keysv = self.player_control.get_keys();
                    for keycode in keysv {
                        if keycode == key {
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
            self.set_state(ProcState::GameOver);
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

    fn process_move_human(&mut self, keycode: VirtualKeyCode) {
        // move player according to the key pressed
        let result = self
            .player_control
            .move_player(&self.obs_table, &self.player, keycode);
        self.player = result;

        // check if we reached a goal
        if self.goal_table.remove_goal_if_reached(self.player.xy()) {
            self.set_state(ProcState::DeliveredPackage(
                self.player.x(),
                self.player.y(),
            ));
        }

        // check if the player has reached all the goals
        if self.goal_table.count() <= 0 {
            self.set_state(ProcState::GameOver);
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
                ProcState::DeliveredPackage(x, y) => ProcState::DeliveredPackage(x, y),
                _ => ProcState::PostMove,
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
        self.set_state(ProcState::Playing);
        self.viewer
            .main_view
            .add_message(&self.obs_table, &self.player, &self.player.recent_event);
        return true;
    }

    fn process_lookmode(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Return => {
                    self.set_state(ProcState::MainMenu);
                }
                _ => {
                    for keycode in self.lookmode.get_keys() {
                        if *keycode == key {
                            let result =
                                self.lookmode
                                    .describe_direction(&self.obs_table, &self.player, *keycode);
                            self.set_state(ProcState::LookedAt(result));
                            break;
                        }
                    }                   
                }
            }
        }

        return true;
    }

    fn process_looked_at(&mut self) -> bool {
        if let ProcState::LookedAt(s) = &self.state {
            self.viewer
                .main_view
                .add_string(String::from(s), RGB::named(rltk::GREEN));
        }
        self.set_state(ProcState::Playing);
        return true;
    }

    fn process_delivered(&mut self, _x: i32, _y: i32) -> bool {
        self.player.n_delivered += 1;

        self.viewer
            .main_view
            .add_string(String::from("Delivered"), RGB::named(rltk::BLUE));

        self.set_state(ProcState::Playing);
        return true;
    }

    fn set_state(&mut self, state: ProcState) {
        self.last_state = ProcState::clone(&self.state);
        self.state = state;
        self.redraw = true;
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
}
