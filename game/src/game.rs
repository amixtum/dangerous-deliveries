use std::collections::HashSet;

use controller::collision;
use model::direction::Direction;
use model::{map_gen};
use rltk::{GameState, RandomNumberGenerator, VirtualKeyCode, RGB, Point};
use util::heap::Heap;

use util::vec_ops;

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

use crate::spawning;

pub struct Game {
    obs_table: ObstacleTable,
    goal_table: GoalTable,

    viewer: ViewManager,

    player_control: PlayerController,

    opponents: Vec<AIController>,

    _lookmode: LookMode,
    lookmode_string: (String, RGB),

    pub player: Player,
    pub recipient_idx: i32,
    pub score: i32,

    pub n_opponents: u32,
    pub ai_sight_radius: u32,
    pub giveup_turns: u32,
    turns_to_giveup: Vec<u32>,
    waiting_to_respawn_idx: HashSet<u32>,
    shirt_colors: [RGB; 8],

    state: ProcState,
    last_state: ProcState,

    redraw: bool,
    first_draw: bool,
    gameover_done: bool,
}

impl Game {
    pub fn new(table_width: u32, table_height: u32) -> Self {
        let mut g = Game {
            obs_table: ObstacleTable::new(table_width, table_height),
            goal_table: GoalTable::new(),

            viewer: ViewManager::new(),

            player_control: PlayerController::new(),
            opponents: Vec::new(),
            _lookmode: LookMode::new(),
            lookmode_string: ("Find the package".to_string(), RGB::named(rltk::WHITE)),

            player: Player::new(table_width as i32 / 2, table_height as i32 / 2),
            recipient_idx: -1,
            score: 0,

            n_opponents: 2,
            ai_sight_radius: 8,
            giveup_turns: 3,
            turns_to_giveup: Vec::new(),
            waiting_to_respawn_idx: HashSet::new(),
            shirt_colors: [RGB::named(rltk::CYAN), RGB::named(rltk::MAGENTA), RGB::named(rltk::AQUAMARINE), RGB::named(rltk::FORESTGREEN),
                                    RGB::named(rltk::TEAL), RGB::named(rltk::BURLYWOOD), RGB::named(rltk::BROWN1), RGB::named(rltk::GRAY)],

            state: ProcState::MainMenu,
            last_state: ProcState::MainMenu,

            redraw: true,
            first_draw: true,
            gameover_done: false,
        };

        Game::init(&mut g);

        return g;
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        // crashes the page on the web 
        /*
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
        */

        let _playing = self.handle_input(ctx);

        /* crashes the webpage
        if !playing {
            std::process::exit(0);
        }
        */

        if self.first_draw {
            ctx.cls();
            self.print_screen(ctx);
            self.first_draw = false;
        } else if self.redraw {
            ctx.cls();
            self.print_screen(ctx);
        }
    }
}

impl Game {
    fn init(g: &mut Game) {
        g.properties_from_file();

        g.reset_game();
    }
    // regen opponent
    fn _add_opponent_tunnel(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let x = (self.obs_table.width() as i32 / 2)
            + rng.range(
                -(self.obs_table.width() as i32) / 2 + 1,
                self.obs_table.width() as i32 / 2,
            )
            - 1;
        let y = (self.obs_table.height() as i32 / 2)
            + rng.range(
                -(self.obs_table.height() as i32) / 2 + 1,
                self.obs_table.height() as i32 / 2 - 1,
            );

        if !(x == self.player.x() && y == self.player.y()) &&
            !self.obs_table.blocked.contains_key(&(x, y)) {
            self.opponents.push(AIController::new(x, y));
            self.turns_to_giveup.push(self.giveup_turns);
            self.obs_table.set_obstacle((x, y), Obstacle::Platform);

            map_gen::tunnel_position(&mut self.obs_table, (x, y));
        }
    }

    fn add_opponent_platform(&mut self) {
        let (x, y) = spawning::random_platform(&self.obs_table);
       
        self.opponents.push(AIController::new(x, y));
        self.turns_to_giveup.push(self.giveup_turns);
    }

    pub fn properties_from_file(&mut self) {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        let raw_data = rltk::embedding::EMBED
            .lock()
            .get_resource(
                "raws/game.txt".to_string(),
            )
            .unwrap();
        let raw_string =
            std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");
        for line in raw_string.lines() {
            if let Some(c) = line.chars().nth(0) {
                if c == '#' {
                    continue;
                }
            } else {
                continue;
            }

            let words: Vec<&str> = line.split_ascii_whitespace().collect();
            if words[0] == "opponents" {
                if let Ok(num) = words[1].parse::<u32>() {
                    self.n_opponents = num;
                }
            } else if words[0] == "ai_sight_radius" {
                if let Ok(num) = words[1].parse::<u32>() {
                    self.ai_sight_radius = num;
                }
            } else if words[0] == "giveup_turns" {
                if let Ok(num) = words[1].parse::<u32>() {
                    self.giveup_turns = num;
                }
            }
        }
    }

    pub fn handle_input(&mut self, ctx: &mut rltk::Rltk) -> bool {
        self.redraw = false;
        self.process(ctx)
    }

    pub fn print_screen(&mut self, ctx: &mut rltk::Rltk) {
        self.viewer.get_screen(
            ctx,
            &self.state,
            &mut self.obs_table,
            &self.goal_table,
            &self.player,
            &self.opponents,
            &self.player_control,
            self.player_control.max_speed,
            self.player_control.fallover_threshold,
            ctx.get_char_size().0,
            ctx.get_char_size().1,
            self.score,
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
            ProcState::Chat => {
                return self.process_chat();
            }
            ProcState::GotPackage(x, y) => {
                return self.process_got_package(x, y);
            }
            ProcState::DeliveredPackage => {
                return self.process_delivered();
            }
            ProcState::LookMode => {
                return self.process_lookmode(ctx);
            }
            ProcState::Restart => {
                return self.process_restart();
            } /*
              _  => {
                  return false;
              },*/
        }
    }

    fn process_chat(&mut self) -> bool {
        let chat_radius = 1;
        let fov = rltk::field_of_view(Point::new(self.player.x(), self.player.y()), chat_radius, &self.obs_table);
        let chat_str = "Stick it to the man".to_string();
        let mut alone = true;
        let mut next_state = ProcState::Playing;
        for p in fov.iter() {
            if p.x == self.player.x() && p.y == self.player.y() {
                continue;
            }

            if self.obs_table.blocked.contains_key(&(p.x, p.y)) {
                alone = false;
                if self.recipient_idx != -1 {
                    if let Some(pack_pos) = self.goal_table.index_map.get(&(self.recipient_idx as usize)) {
                        if !self.goal_table.picked_up.contains(pack_pos) {
                            let dir = vec_to_direction(pack_pos.0 - p.x, pack_pos.1 - p.y);
                            let mut message = "There's a job ".to_string();
                            let dirstr = self.direction_string(dir);
                            match &dirstr.1 {
                                None => {
                                    message.push_str(&dirstr.0);
                                    self.viewer.main_view.add_string(message, RGB::named(rltk::GREEN));
                                }
                                Some(_) => {
                                    message.push_str("here");
                                    self.viewer.main_view.add_string(message, RGB::named(rltk::GREEN));
                                }
                            }
                        }
                        else {
                            let recv_pos = self.opponents[self.recipient_idx as usize].player.position;
                            let dir = vec_to_direction(recv_pos.0 - p.x, recv_pos.1 - p.y);
                            let mut message = "Last saw them ".to_string();
                            let dirstr = self.direction_string(dir);
                            match &dirstr.1 {
                                None => {
                                    message.push_str(&dirstr.0);
                                    self.viewer.main_view.add_string(message, RGB::named(rltk::GREEN));
                                }
                                Some(score) => {
                                    self.score += score;
                                    self.viewer.main_view.add_string(dirstr.0, RGB::named(rltk::GREEN));
                                    next_state = ProcState::DeliveredPackage;
                                }
                            }
                        }
                    }
                }
            }
        }

        if alone {
            self.viewer.main_view.add_string(chat_str, RGB::named(rltk::DARKGRAY));
        }

        self.set_state(next_state);

        return true;
    }



    fn direction_string(&self, dir: Direction) -> (String, Option<i32>) {
        match dir {
            Direction::Center => {
                let score = (((self.obs_table.width() as f32 * self.obs_table.height() as f32) / self.player.time as f32))
                .round() as i32;
                return (format!("Thanks. Here's ${} for your hard work", score), Some(score));
            }
            Direction::Down => ("to the south".to_string(), None),
            Direction::Up => ("to the north".to_string(), None),
            Direction::Left => ("to the west".to_string(), None),
            Direction::Right => ("to the east".to_string(), None),
            Direction::NorthEast => ("to the northeast".to_string(), None),
            Direction::NorthWest => ("to the northwest".to_string(), None),
            Direction::SouthEast => ("to the southeast".to_string(), None),
            Direction::SouthWest => ("to the southwest".to_string(), None),
        }
    }

    fn process_delivered(&mut self) -> bool {
        let mut rng = RandomNumberGenerator::new();

        let pos = self.goal_table.index_map.get(&(self.recipient_idx as usize));
        match pos {
            None => {}
            Some(pos) => {
                self.goal_table.picked_up.remove(&*pos);
                self.goal_table.remove_goal_if_reached((pos.0, pos.1));
            }
        }

        self.lookmode_string = ("Find the package".to_string(), RGB::named(rltk::WHITE));

        // for computing the player's score
        self.player.n_delivered += 1;

        // spawn a new package
        let mut aiidx = rng.range(0, self.opponents.len());
        while aiidx == self.recipient_idx as usize {
            aiidx = rng.range(0, self.opponents.len());
        }
        self.recipient_idx = aiidx as i32;

        let coloridx = rng.range(0, self.shirt_colors.len());

        self.goal_table.add_goal(
            spawning::random_platform(&mut self.obs_table), 
            (aiidx, self.shirt_colors[coloridx]));

        self.set_state(ProcState::Playing);

        return true;
    }

    fn process_main_menu(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Q => {
                    return false;
                }
                VirtualKeyCode::Escape => {
                    self.set_state(ProcState::Help);
                }
                VirtualKeyCode::Return => {
                    self.set_state(match self.last_state {
                        ProcState::LookMode => ProcState::LookMode,
                        _ => ProcState::Playing,
                    });
                }
                _ => {}
            },
        }

        return true;
    }

    fn process_help(&mut self, ctx: &mut rltk::Rltk) -> bool {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    self.set_state(ProcState::MainMenu);
                }
                _ => {}
            },
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
                _ => {}
            },
        }

        return true;
    }

    fn process_restart(&mut self) -> bool {
        self.reset_game();
        self.set_state(ProcState::Playing);
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
                    self.set_state(ProcState::LookMode);
                }
                VirtualKeyCode::G => {
                    self.set_state(ProcState::Chat);
                }
                VirtualKeyCode::Key5 => {
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
                                (100.0 / (1.0 + vec_ops::magnitude(self.player.speed))) as u32,
                                (999, PlayerType::Human),
                            );

                            // insert the ai opponents
                            for index in 0..self.opponents.len() {
                                heap.insert(
                                    (100.0 / (1.0 + vec_ops::magnitude(self.opponents[index].player.speed)))
                                        as u32,
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
                                    &self.waiting_to_respawn_idx
                                );
                            }
                            break;
                        }
                    }
                }
            },
        }

        return true;
    }

    fn process_ai(&mut self, index: usize) {
        if self.waiting_to_respawn_idx.contains(&(index as u32)) {
            let mut rng = RandomNumberGenerator::new();
            let x = (self.obs_table.width() as i32 / 2)
                + rng.range(
                    -(self.obs_table.width() as i32) / 2 + 1,
                    self.obs_table.width() as i32 / 2,
                )
                - 1;
            let y = (self.obs_table.height() as i32 / 2)
                + rng.range(
                    -(self.obs_table.height() as i32) / 2 + 1,
                    self.obs_table.height() as i32 / 2 - 1,
                );

            if !self.obs_table.blocked.contains_key(&(x, y)) &&
                self.obs_table.get_obstacle(x, y) == Obstacle::Platform {
                // we found an empty space to respawn
                self.opponents[index].player = PlayerController::reset_ai_continue(&self.opponents[index].player, x, y);
                self.opponents[index].choose_goal(&self.obs_table, self.ai_sight_radius);
                self.turns_to_giveup[index] = self.giveup_turns;
                self.waiting_to_respawn_idx.remove(&(index as u32));
            }
            return;
        }

        if self.opponents[index].goal.0 == -1 || self.opponents[index].goal.1 == -1 {
            self.opponents[index].choose_goal(&self.obs_table, self.ai_sight_radius);
        }
        let last_pos = self.opponents[index].player.position;

        self.opponents[index].move_player(&self.obs_table, &self.player_control);
        let new_pos = self.opponents[index].player.position;

        if last_pos.0 == new_pos.0 && last_pos.1 == new_pos.1 {
            self.turns_to_giveup[index] -= 1;
        }

        if self.opponents[index].reached_goal(self.ai_sight_radius as f32 - self.ai_sight_radius as f32 / 3.0) || self.turns_to_giveup[index] <= 0 {
            self.opponents[index].choose_goal(&self.obs_table, self.ai_sight_radius);
            self.turns_to_giveup[index] = self.giveup_turns;
        }
        else if let PlayerEvent::GameOver(_) = self.opponents[index].player.recent_event {
            self.waiting_to_respawn_idx.insert(index as u32);
        } 
        else if self.opponents[index].player.recent_event == PlayerEvent::Respawn {
            self.waiting_to_respawn_idx.insert(index as u32);
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
        if self.goal_table.at_goal(self.player.xy()) && !self.goal_table.picked_up.contains(&self.player.xy()) {
            self.set_state(ProcState::GotPackage(
                self.player.x(),
                self.player.y(),
            ));
        }

        // check if move player returned a player with a Respawn event
        // and respawn them
        if self.player.recent_event == PlayerEvent::Respawn  {
            self.reset_player_continue();
        }

        // go to the state where we update the message log
        // after computing the result of the turn
        else {
            self.set_state(match self.state {
                ProcState::GotPackage(x, y) => ProcState::GotPackage(x, y),
                _ => ProcState::PostMove,
            });
        }
    }

    // dummy state for the purposes of updating the view after process_move
    fn process_post_move(&mut self) -> bool {
        self.set_state(ProcState::Playing);

        return true;
    }

    fn process_lookmode(&mut self, _ctx: &mut rltk::Rltk) -> bool {
        self.viewer.main_view.add_string(String::clone(&self.lookmode_string.0), self.lookmode_string.1);

        self.set_state(ProcState::Playing);

        return true;
    }

    fn process_got_package(&mut self, x: i32, y: i32) -> bool {
        if let Some(idx_color) = self.goal_table.goals.get(&(x, y)) {
            self.recipient_idx = idx_color.0 as i32;
            self.viewer
                .main_view
                .add_string(String::from("Picked up package, find the skater wearing this color shirt"), idx_color.1);

            self.lookmode_string = ("Find the skater wearing this color shirt".to_string(), idx_color.1);           

            self.goal_table.picked_up.insert((x, y));
        }


        self.set_state(ProcState::Playing);

        return true;
    }

    fn set_state(&mut self, state: ProcState) {
        self.last_state = ProcState::clone(&self.state);
        self.state = state;
        self.redraw = true;
    }

    fn reset_game(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.lookmode_string = ("Find the package".to_string(), RGB::named(rltk::WHITE));

        self.obs_table.revelead.clear();
        self.obs_table.regen_table();

        map_gen::voronoi_mapgen(&mut self.obs_table);

        self.obs_table.update_platforms();

        self.obs_table.compute_unions();

        map_gen::tunnel_pockets(&mut self.obs_table);

        self.obs_table.update_platforms();

        self.opponents.clear();
        self.turns_to_giveup.clear();
        for _ in 0..self.n_opponents {
            self.add_opponent_platform();
        }

        self.goal_table.clear();
        let mut aiidx = rng.range(0, self.opponents.len()) as i32;
        while aiidx == self.recipient_idx {
            aiidx = rng.range(0, self.opponents.len() as i32);
        }
        self.recipient_idx = aiidx;

        let coloridx = rng.range(0, self.shirt_colors.len());
        self.goal_table.add_goal(
            spawning::random_platform(&self.obs_table), 
            (aiidx as usize, self.shirt_colors[coloridx]));


        let (x, y) = spawning::tunnel_spawn(&mut self.obs_table);

        self.obs_table.update_platforms();

        self.player = PlayerController::reset_player_gameover(&self.obs_table, &self.player, x, y);

        collision::update_blocked(&mut self.obs_table, &self.player, &self.opponents, &self.waiting_to_respawn_idx);
    }

    fn reset_player_continue(&mut self) {
        let spawn_at = spawning::random_platform(&self.obs_table);
        self.player = PlayerController::reset_player_continue(&self.obs_table, &self.player, spawn_at.0, spawn_at.1);
        self.redraw = true;
    }
}


fn vec_to_direction(x: i32, y: i32,) -> Direction {
    if x.abs() >= y.abs() {
        if x < 0 {
            return Direction::Left;
        }
        else if x > 0 {
            return Direction::Right;
        }
        else {
            return Direction::Center;
        }
    } 
    else {
        if y < 0 {
            return Direction::Up;
        }
        else if y > 0 {
            return Direction::Down;
        }
        else {
            return Direction::Center;
        }
    }
}