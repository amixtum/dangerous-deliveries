use console_engine::KeyCode;

use rand::Rng;

use std::collections::HashMap;
use std::fs;
//use std::f32::consts::PI;

use model::obstacle::{Obstacle};
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;

use util::vec_ops;

pub struct PlayerController {
    key_map: HashMap<KeyCode, (f32, f32)>,
    pub speed_damp: f32,
    pub balance_damp: f32,
    pub turn_factor: f32,
    pub onrail_balance_factor: f32,
    pub offrail_balance_factor: f32,
    pub up_speed_factor: f32,
    pub down_speed_factor: f32,
    pub max_speed: f32,
    pub fallover_threshold: f32,
    pub inst_length: f32,
    pub rail_boost: f32,
}

impl PlayerController {
    pub fn new(conf_file: &str) -> Self {
        let mut speed_damp: f32 = 0.5;
        let mut inst_length: f32 = 0.66;
        let mut rail_boost: f32 = 2.0;
        let mut balance_damp: f32 = 0.75;
        let mut turn_factor: f32 = 0.25;
        let mut onrail_balance_factor: f32 = 0.25;
        let mut offrail_balance_factor: f32 = 0.25;
        let mut up_speed_factor: f32 = 0.5;
        let mut down_speed_factor: f32 = 1.5;
        let mut max_speed: f32 = 3.0;
        let mut fallover_threshold: f32 = 5.0;

        // read conf file
        if let Ok(contents) = fs::read_to_string(conf_file) {
            for line in contents.lines() {
                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                if words.len() == 2 {
                    if words[0] == "rail_boost" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            rail_boost = num;
                        }
                    }
                    if words[0] == "inst_length" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            inst_length = num;
                        }
                    }
                    if words[0] == "speed_damp" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            speed_damp = num;
                        }
                    } else if words[0] == "balance_damp" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            balance_damp = num;
                        }
                    } else if words[0] == "turn_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            turn_factor = num;
                        }
                    } else if words[0] == "offrail_balance_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            offrail_balance_factor = num;
                        }
                    }
                    if words[0] == "turn_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            turn_factor = num;
                        }
                    } else if words[0] == "onrail_balance_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            onrail_balance_factor = num;
                        }
                    } else if words[0] == "up_speed_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            up_speed_factor = num;
                        }
                    } else if words[0] == "down_speed_factor" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            down_speed_factor = num;
                        }
                    } else if words[0] == "max_speed" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            max_speed = num;
                        }
                    } else if words[0] == "fallover_threshold" {
                        if let Ok(num) = words[1].parse::<f32>() {
                            fallover_threshold = num;
                        }
                    }
                }
            }
        }

        let mut pc = PlayerController {
            key_map: HashMap::new(),
            speed_damp,
            balance_damp,
            turn_factor,
            onrail_balance_factor,
            offrail_balance_factor,
            up_speed_factor,
            down_speed_factor,
            max_speed,
            fallover_threshold,
            inst_length,
            rail_boost,
        };

        // left
        pc.key_map.insert(KeyCode::Char('h'), (-1.0, 0.0));
        pc.key_map.insert(KeyCode::Char('a'), (-1.0, 0.0));

        // right
        pc.key_map.insert(KeyCode::Char('l'), (1.0, 0.0));
        pc.key_map.insert(KeyCode::Char('d'), (1.0, 0.0));

        // up
        pc.key_map.insert(KeyCode::Char('k'), (0.0, -1.0));
        pc.key_map.insert(KeyCode::Char('w'), (0.0, -1.0));

        // down
        pc.key_map.insert(KeyCode::Char('j'), (0.0, 1.0));
        pc.key_map.insert(KeyCode::Char('s'), (0.0, 1.0));

        // up right
        pc.key_map.insert(KeyCode::Char('u'), (1.0, -1.0));
        pc.key_map.insert(KeyCode::Char('e'), (1.0, -1.0));

        // up left
        pc.key_map.insert(KeyCode::Char('y'), (-1.0, -1.0));
        pc.key_map.insert(KeyCode::Char('q'), (-1.0, -1.0));

        // down left
        pc.key_map.insert(KeyCode::Char('b'), (-1.0, 1.0));
        pc.key_map.insert(KeyCode::Char('z'), (-1.0, 1.0));

        // down right
        pc.key_map.insert(KeyCode::Char('n'), (1.0, 1.0));
        pc.key_map.insert(KeyCode::Char('c'), (1.0, 1.0));

        // wait
        pc.key_map.insert(KeyCode::Char('.'), (0.0, 0.0));
        pc.key_map.insert(KeyCode::Tab, (0.0, 0.0));

        pc
    }
}

impl PlayerController {
    pub fn set_model(
        &mut self,
        speed_damp: f32,
        balance_damp: f32,
        turn_factor: f32,
        onrail_balance_factor: f32,
        offrail_balance_factor: f32,
        up_speed_factor: f32,
        down_speed_factor: f32,
        max_speed: f32,
        fallover_threshold: f32,
    ) {
        self.speed_damp = speed_damp;
        self.balance_damp = balance_damp;
        self.turn_factor = turn_factor;
        self.onrail_balance_factor = onrail_balance_factor;
        self.offrail_balance_factor = offrail_balance_factor;
        self.up_speed_factor = up_speed_factor;
        self.down_speed_factor = down_speed_factor;
        self.max_speed = max_speed;
        self.fallover_threshold = fallover_threshold;
    }

    pub fn get_keys(&self) -> Vec<KeyCode> {
        self.key_map.keys().map(|k| k.clone()).collect()
    }

    pub fn get_inst_velocity(&self, key: KeyCode) -> Option<&(f32, f32)> {
        self.key_map.get(&key)
    }

    pub fn move_player_vel(&self, table: &ObstacleTable, player: &Player, inst_v: (f32, f32)) -> Player {
        return PlayerController::compute_move(
            table,
            player,
            inst_v,
            self.speed_damp,
            self.balance_damp,
            self.turn_factor,
            self.up_speed_factor,
            self.down_speed_factor,
            self.max_speed,
            self.fallover_threshold,
            self.inst_length,
            self.rail_boost,
        );
    }

    pub fn move_player(&self, table: &ObstacleTable, player: &Player, key: KeyCode) -> Player {
        if let Some(inst_v) = self.get_inst_velocity(key) {
            return PlayerController::compute_move(
                table,
                player,
                *inst_v,
                self.speed_damp,
                self.balance_damp,
                self.turn_factor,
                self.up_speed_factor,
                self.down_speed_factor,
                self.max_speed,
                self.fallover_threshold,
                self.inst_length,
                self.rail_boost,
            );
        }
        return PlayerController::compute_move(
            table,
            player,
            (0.0, 0.0),
            self.speed_damp,
            self.balance_damp,
            self.turn_factor,
            self.up_speed_factor,
            self.down_speed_factor,
            self.max_speed,
            self.fallover_threshold,
            self.inst_length,
            self.rail_boost,
        );
    }

    pub fn reset_player_gameover(table: &ObstacleTable, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.position = (
            table.width() as i32 / 2,
            table.height() as i32 / 2,
        );
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.recent_event = PlayerEvent::GameOver(clone.time.round() as i32);
        clone.time = 0.0;
        clone.n_falls = 0;
        clone.n_delivered = 0;
        clone
    }

    pub fn reset_player_continue(table: &ObstacleTable, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        let last_pos = clone.position;
        clone.position = (
            table.width() as i32 / 2,
            table.height() as i32 / 2,
        );
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.n_falls = 0;
        clone.time += vec_ops::magnitude((
            clone.x() as f32 - last_pos.0 as f32,
            clone.y() as f32 - last_pos.1 as f32,
        )) / 2.0;
        clone
    }

    pub fn reset_ai_continue(table: &ObstacleTable, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        let mut x = rand::thread_rng().gen_range(
            (table.width() as i32 / 2 - table.width() as i32 / 8)
                ..(table.width() as i32 / 2 + table.width() as i32 / 8),
        );
        let mut y = rand::thread_rng().gen_range(
            (table.height() as i32 / 2 - table.height() as i32 / 8)
                ..(table.height() as i32 / 2 + table.height() as i32 / 8),
        );

        while x == table.width() as i32 / 2 && y == table.height() as i32 / 2 {
            x = rand::thread_rng().gen_range(
                (table.width() as i32 / 2 - table.width() as i32 / 8)
                    ..(table.width() as i32 / 2 + table.width() as i32 / 8),
            );
            y = rand::thread_rng().gen_range(
                (table.height() as i32 / 2 - table.height() as i32 / 8)
                    ..(table.height() as i32 / 2 + table.height() as i32 / 8),
            );
        }
        clone.position = (x, y);
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.n_falls = 0;
        clone
    }

    pub fn compute_move(
        table: &ObstacleTable,
        player: &Player,
        (inst_x, inst_y): (f32, f32),
        speed_damp: f32,
        balance_damp: f32,
        turn_fact: f32,
        up_speed_fact: f32,
        down_speed_fact: f32,
        max_speed: f32,
        fallover_threshold: f32,
        inst_length: f32,
        rail_boost: f32,
    ) -> Player {
        // compute initial speed and balance values
        let mut player = PlayerController::compute_initial_speed_balance(
            table,
            player,
            (inst_x, inst_y),
            max_speed,
            speed_damp,
            balance_damp,
            turn_fact,
            inst_length,
            rail_boost,
        );

        // compute position and updated player fields
        let result = PlayerController::compute_next_position(table, &player, (inst_x, inst_y));

        player = result.0;
        let next_pos = result.1;

        player.time += 1.0 / (1.0 + vec_ops::magnitude(player.speed));

        match player.recent_event {
            PlayerEvent::FallOver => {
                return player;
            }
            _ => {}
        }

        // fallover if the player is off balance
        if vec_ops::magnitude(player.balance) >= fallover_threshold
        {
            return PlayerController::fallover(table, &player);
        }

        // fall into a pit. Game Over
        if let Obstacle::Pit = table.get_obstacle(next_pos.0, next_pos.1) {
            // reset the player
            return PlayerController::reset_player_continue(table, &player);
        }

        if let Obstacle::Wall = table.get_obstacle(next_pos.0, next_pos.1) {
            return PlayerController::fallover(table, &player);
        }

        // try to move player to next_pos
        // updating speed values depending on the change
        // in height after the move
        // and return the updated Player
        return PlayerController::try_traverse(
            table,
            &player,
            next_pos,
            up_speed_fact,
            down_speed_fact,
        );
    }

    fn fallover(table: &ObstacleTable, player: &Player) -> Player {
        let mut clone = Player::clone(player);
        clone.recent_event = PlayerEvent::FallOver;
        clone.speed = (0.0, 0.0);
        clone.balance = (0.0, 0.0);
        clone.n_falls += 1;

        match table.get_obstacle(player.x(), player.y()) {
            Obstacle::Rail(_, _) => {
                let neighbors = vec_ops::neighbors(
                    player.xy(),
                    (0, 0),
                    (table.width() as i32 - 1, table.height() as i32 - 1),
                );
                let mut found = false;

                for neighbor in neighbors {
                    match table.get_obstacle(neighbor.0, neighbor.1) {
                        Obstacle::Platform => {
                                clone.position = (neighbor.0, neighbor.1);
                                found = true;
                                break;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                if !found {
                    clone = PlayerController::reset_player_continue(table, player);
                }
            }
            _ => {}
        }

        clone
    }

    fn get_scaled((inst_x, inst_y): (f32, f32), inst_length: f32) -> (f32, f32) {
        let norm_inst = vec_ops::normalize((inst_x, inst_y));
        if !f32::is_nan(norm_inst.0) {
            let units = vec_ops::discrete_jmp(norm_inst);
            if inst_length.abs() < 1.0 {
                if units.0 == 1 && units.1 == 1 {
                    return (norm_inst.0 * inst_length.sqrt(),
                            norm_inst.1 * inst_length.sqrt());
                } else {
                    return (norm_inst.0 * inst_length,
                            norm_inst.1 * inst_length);
                }
            } else {
                if units.0 == 1 && units.1 == 1 {
                    return (norm_inst.0 * inst_length,
                            norm_inst.1 * inst_length);
                } else {
                    return (norm_inst.0 * inst_length.sqrt(),
                            norm_inst.1 * inst_length.sqrt());
                }
            }
        }
        (0.0, 0.0)
    }
    fn compute_initial_speed_balance(
        table: &ObstacleTable,
        player: &Player,
        (inst_x, inst_y): (f32, f32),
        max_speed: f32,
        speed_damp: f32,
        balance_damp: f32,
        turn_fact: f32,
        inst_length: f32,
        rail_boost: f32,
    ) -> Player {
        let norm_inst = vec_ops::normalize((inst_x, inst_y));
        let last_speed = player.speed;
        let last_obstacle = table.get_obstacle(player.position.0, player.position.1);

        let mut clone = Player::clone(player);

        match last_obstacle {
            Obstacle::Platform => {
                // compute speed
                clone.speed.0 = clone.speed.0 * speed_damp;
                clone.speed.1 = clone.speed.1 * speed_damp;

                let add = PlayerController::get_scaled((inst_x, inst_y), inst_length);
                clone.speed.0 += add.0; 
                clone.speed.1 += add.1;
            }
            Obstacle::Rail(x_dir, y_dir) => {
                // compute speed

                // this will not be NaN, if it is it's a bug
                // found two on the borders
                let norm_dir = vec_ops::normalize((x_dir as f32, y_dir as f32));

                clone.speed.0 = clone.speed.0 * speed_damp;
                clone.speed.1 = clone.speed.1 * speed_damp;

                if !f32::is_nan(norm_dir.0) {
                    clone.speed.0 += norm_dir.0 * rail_boost;
                    clone.speed.1 += norm_dir.1 * rail_boost;
                }

                let inst_add = PlayerController::get_scaled((inst_x, inst_y), inst_length);

                clone.speed.0 += inst_add.0;
                clone.speed.1 += inst_add.1;


                clone.speed.0 = clone.speed.0.clamp(-max_speed, max_speed);
                clone.speed.1 = clone.speed.1.clamp(-max_speed, max_speed);
            }
            _ => {}
        }

        if vec_ops::magnitude(clone.speed) >= max_speed {
            clone.speed.0 *= max_speed / vec_ops::magnitude(clone.speed);
            clone.speed.1 *= max_speed / vec_ops::magnitude(clone.speed);
        }
        else if vec_ops::magnitude(clone.speed) <= 0.25 {
            clone.speed.0 = 0.0;
            clone.speed.1 = 0.0;
        }

        let norm_last_speed = vec_ops::normalize(last_speed);

        if !f32::is_nan(norm_inst.0) && !f32::is_nan(norm_last_speed.0) {
            let inst_v = PlayerController::get_scaled((inst_x, inst_y), inst_length);

            let diff = (last_speed.0 - inst_v.0, last_speed.1 - inst_v.1);

            let dotp = vec_ops::dot(inst_v, last_speed).abs();

            let turn = (vec_ops::magnitude(inst_v) * vec_ops::magnitude(last_speed)
                - dotp) / (vec_ops::magnitude(inst_v) * vec_ops::magnitude(last_speed));
            
            clone.balance.0 =
                clone.balance.0 * balance_damp + 
                diff.1.signum() as f32 * turn * turn_fact;

            clone.balance.1 =
                clone.balance.1 * balance_damp - 
                diff.0.signum() as f32 * turn * turn_fact;

            //clone.balance.0 += norm_inst.0 * turn_fact;
            //clone.balance.1 += norm_inst.1 * turn_fact;
        } else {
            clone.balance.0 = clone.balance.0 * balance_damp;
            clone.balance.1 = clone.balance.1 * balance_damp;

            /*
            if !f32::is_nan(norm_inst.0) {
                clone.balance.0 += norm_inst.0 * turn_fact;
                clone.balance.1 += norm_inst.1 * turn_fact;
            }
            */
        }

        if vec_ops::magnitude(clone.balance) <= 0.25 {
            clone.balance = (0.0, 0.0);
        }

        clone
    }

    /*
    // updated a player's balance so it must return a new player as well
    fn compute_onrail(table: &ObstacleTable, player: &Player, (inst_x, inst_y): (f32, f32), (x_dir, y_dir): (f32, f32), onrail_balance_fact: f32, rail_boost: f32) -> (Player, (i32, i32, i32)) {
        let (unit_x, unit_y) = vec_ops::discrete_jmp((inst_x, inst_y));
        let mut next_pos = player.position;
        next_pos.0 = (player.position.0 + unit_x).clamp(0, table.width() as i32 - 1);
        next_pos.1 = (player.position.1 + unit_y).clamp(0, table.height() as i32 - 1);
        next_pos.2 = table.get_height(next_pos.0, next_pos.1);

        let mut clone = Player::clone(player);

        let norm_speed = vec_ops::normalize(clone.speed);
        let norm_dir = vec_ops::normalize((x_dir, y_dir));

        if !f32::is_nan(norm_speed.0) && !f32::is_nan(norm_dir.0) {
            let scaled_dir = (x_dir * rail_boost, y_dir * rail_boost);
            let turn = vec_ops::magnitude(scaled_dir)*vec_ops::magnitude(clone.speed) - vec_ops::dot(clone.speed, scaled_dir);
            clone.balance.0 += norm_dir.1.signum() *
                               turn *
                               onrail_balance_fact;

            clone.balance.1 += norm_dir.0.signum() *
                               turn *
                               onrail_balance_fact;
        }

        (clone, next_pos)
    }
    */

    fn compute_continue(table: &ObstacleTable, player: &Player) -> (i32, i32) {
        let mut next_pos = player.position;

        next_pos.0 = ((next_pos.0 as f32 + player.speed.0).round() as i32)
            .clamp(next_pos.0 - 1, next_pos.0 + 1);
        next_pos.0 = next_pos.0.clamp(0, table.width() as i32 - 1);

        next_pos.1 = ((next_pos.1 as f32 + player.speed.1).round() as i32)
            .clamp(next_pos.1 - 1, next_pos.1 + 1);
        next_pos.1 = next_pos.1.clamp(0, table.height() as i32 - 1);

        next_pos
    }

    // Note: Must call compute_intitial_speed_balance first in order to update speed and balance
    // values, otherwise, this will compute the next position without taking into account user
    // input
    fn compute_next_position(
        table: &ObstacleTable,
        player: &Player,
        (inst_x, inst_y): (f32, f32),
    ) -> (Player, (i32, i32)) {
        let mut next_pos = player.position;
        let last_obstacle = table.get_obstacle(player.x(), player.y());
        let units = vec_ops::discrete_jmp((inst_x, inst_y));
        let unit_x = units.0;
        let unit_y = units.1;

        let mut clone = Player::clone(player);
        clone.recent_event = PlayerEvent::Move;

        // bump into border
        if player.x() + unit_x >= table.width() as i32
            || player.x() + unit_x < 0
            || player.y() + unit_y >= table.height() as i32
            || player.y() + unit_y < 0
        {
            clone = PlayerController::fallover(table, player);
            return (clone, clone.position);
        }

        // compute position
        let obs_at_next = table.get_obstacle(player.x() + unit_x, player.y() + unit_y);
        match last_obstacle {
            Obstacle::Rail(last_height, _) => match obs_at_next {
                Obstacle::Rail(height, _) => {
                    if (height - last_height).abs() > 1
                        || vec_ops::magnitude(player.speed).abs() < 0.1
                    {
                        clone = PlayerController::fallover(table, player);
                    } else {
                        next_pos = PlayerController::compute_continue(table, player);
                        clone.recent_event = PlayerEvent::OffRail;
                    }
                }
                _ => {
                    next_pos = PlayerController::compute_continue(table, player);
                    clone.recent_event = PlayerEvent::OnRail;
                }
            },
            Obstacle::Platform => {
                match obs_at_next {
                    Obstacle::Rail(_x_dir, _y_dir) => {
                        /*
                        let result = PlayerController::compute_onrail(
                            table,
                            player,
                            (inst_x, inst_y),
                            (x_dir, y_dir),
                            onrail_balance_fact,
                            rail_boost,
                        );
                        clone = result.0;
                        next_pos = result.1
                        clone.recent_event = PlayerEvent::OnRail;
                        */
                        next_pos = PlayerController::compute_continue(table, player);
                    }
                    _ => {
                        next_pos = PlayerController::compute_continue(table, player);
                    }
                }
            }

            _ => {}
        }

        if next_pos.0 == player.x() && next_pos.1 == player.y() {
            clone.recent_event = PlayerEvent::Wait;
            return (clone, next_pos);
        }

        if next_pos.0 >= table.width() as i32
            || next_pos.0 < 0
            || next_pos.1 >= table.height() as i32
            || next_pos.1 < 0
        {
            clone = PlayerController::fallover(table, &clone);
            return (clone, clone.position);
        }

        (clone, next_pos)
    }

    fn try_traverse(
        table: &ObstacleTable,
        player: &Player,
        next_pos: (i32, i32),
        _up_speed_fact: f32,
        _down_speed_fact: f32,
    ) -> Player {
        // check if next_pos is adjacent to current position
        let mut clone = Player::clone(player);
        let last_obstacle = table.get_obstacle(clone.x(), clone.y());

        if table.can_traverse(player.xy(), (next_pos.0, next_pos.1)) {
            if clone.x() != next_pos.0 || clone.y() != next_pos.1 {
                // move player to next position
                clone.position.0 = next_pos.0.clamp(0, table.width() as i32 - 1);
                clone.position.1 = next_pos.1.clamp(0, table.height() as i32 - 1);

                match table.get_obstacle(clone.x(), clone.y()) {
                    Obstacle::Platform => match last_obstacle {
                        Obstacle::Platform => {
                            clone.recent_event = PlayerEvent::Move;
                        }
                        Obstacle::Pit => {}
                        Obstacle::Rail(_, _) => {
                            clone.recent_event = PlayerEvent::OffRail;
                        }
                        _ => {}
                    },
                    Obstacle::Pit => {
                        clone.recent_event = PlayerEvent::GameOver(clone.time.round() as i32);
                    }
                    Obstacle::Rail(_, _) => {
                        clone.recent_event = PlayerEvent::OnRail;
                    }
                    _ => {}
                }
            }

        // fallover if we cannot traverse to next_pos
        // and do not update the player's position
        } else {
            clone = PlayerController::fallover(table, player);
        }

        clone
    }
}
