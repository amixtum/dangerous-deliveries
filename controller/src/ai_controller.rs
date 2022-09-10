use rand::prelude::SliceRandom;
use rand::Rng;

use super::player_controller::PlayerController;

use model::goal_table::GoalTable;
use model::obstacle::ObstacleType;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;

use util::vec_ops;

pub struct AIController {
    pub player: Player,
    pub goal: (i32, i32),
}

impl AIController {
    pub fn new(goal_table: &GoalTable, start_x: i32, start_y: i32, height: i32) -> Self {
        let mut ac = AIController {
            player: Player::new(start_x, start_y, height),
            goal: (-1, -1),
        };
        ac.choose_goal(goal_table);
        ac
    }
}

impl AIController {
    pub fn reset(&mut self, obs_table: &ObstacleTable, x: i32, y: i32) {
        self.player.position.0 = x;
        self.player.position.1 = y;
        self.player.position.2 = obs_table.get_height(x, y);
        self.player.balance = (0.0, 0.0);
        self.player.speed = (0.0, 0.0);
        self.player.n_falls = 0;
        self.player.recent_event = PlayerEvent::GameOver(self.player.time.round() as i32);
    }

    pub fn choose_goal(&mut self, goal_table: &GoalTable) {
        let mut rng = rand::thread_rng();
        let v: Vec<&(i32, i32)> = goal_table.goals().iter().collect();
        if let Ok(choice) = v.choose_weighted(&mut rng, |item| {
            return 1.0
                / vec_ops::magnitude((
                    item.0 as f32 - self.player.x() as f32,
                    item.1 as f32 - self.player.y() as f32,
                ));
        }) {
            self.goal = **choice;
        }
    }

    pub fn move_player(&mut self, obs_table: &ObstacleTable, player_control: &PlayerController) {
        self.player = self.next_move(obs_table, player_control);
    }

    pub fn reached_goal(&self) -> bool {
        self.player.x() == self.goal.0 && self.player.y() == self.goal.1
    }

    pub fn next_move(
        &mut self,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Player {
        let try_platform = self.next_platform(obs_table, player_control);

        // if the only platform is the one the player is standing on,
        // try again, this time including rails
        // dont' exclude the possibility of staying in place indefinitely
        if try_platform.x() == self.player.x() && try_platform.y() == self.player.y() {
            let mut moves = AIController::get_moves(&self.player, obs_table, player_control);
            moves.sort_by(|l, r| self.dist_to_goal(&l.0).cmp(&self.dist_to_goal(&r.0)));

            if moves.len() > 0 {
                if rand::thread_rng().gen_bool(0.77) || moves.len() == 1 {
                    return moves[0].0;
                } else if moves.len() > 1 {
                    return moves[1].0;
                }
            }
        }

        return try_platform;
    }

    fn next_platform(
        &mut self,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Player {
        let mut moves = AIController::get_moves_platform(&self.player, obs_table, player_control);

        if moves.len() > 0 {
            moves.sort_by(|l, r| self.dist_to_goal(&l.0).cmp(&self.dist_to_goal(&r.0)));

            if rand::thread_rng().gen_bool(0.77) || moves.len() == 1 {
                return moves[0].0;
            } else if moves.len() > 1 {
                return moves[1].0;
            }
        }

        self.player.recent_event = PlayerEvent::GameOver(self.player.time.round() as i32);

        return self.player;
    }

    fn get_moves(
        player: &Player,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Vec<(Player, u32)> {
        let mut moves = Vec::new();
        let mut falls: Vec<(Player, u32)> = Vec::new();

        // iterate through all possible inputs to the player controller
        // and push the new player with the time the move took
        for key in player_control.get_keys() {
            let mov = player_control.move_player(&obs_table, &player, key);

            match mov.recent_event {
                PlayerEvent::GameOver(_) => {}
                PlayerEvent::FallOver => {
                    falls.push((mov, (mov.time - player.time).round() as u32));
                }
                _ => match obs_table.get_obstacle_type(mov.x(), mov.y()) {
                    ObstacleType::Pit => {}
                    _ => {
                        moves.push((mov, (mov.time - player.time).round() as u32));
                    }
                },
            }
        }

        if moves.len() == 0 {
            let mut rng = rand::thread_rng();
            if let Some(choice) = falls.choose(&mut rng) {
                moves.push(*choice);
            }
        }

        moves
    }

    fn get_moves_platform(
        player: &Player,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Vec<(Player, u32)> {
        let mut moves = Vec::new();
        let mut falls = Vec::new();

        // iterate through the possible inputs and push any moves that end on a platform
        // along with the time it took to make the move
        for key in player_control.get_keys() {
            let mov = player_control.move_player(&obs_table, &player, key);
            match mov.recent_event {
                PlayerEvent::GameOver(_) => {}
                PlayerEvent::FallOver => {
                    falls.push((mov, (mov.time - player.time).round() as u32));
                }
                _ => match obs_table.get_obstacle_type(mov.x(), mov.y()) {
                    ObstacleType::Platform => {
                        moves.push((mov, (mov.time - player.time).round() as u32));
                    }
                    _ => {}
                },
            }
        }

        if moves.len() == 0 {
            let mut rng = rand::thread_rng();
            if let Some(choice) = falls.choose(&mut rng) {
                moves.push(*choice);
            }
        }

        moves
    }

    fn dist_to_goal(&self, player: &Player) -> u32 {
        let diff = (
            (self.goal.0 - player.x()) as f32,
            (self.goal.1 - player.y()) as f32,
        );
        return vec_ops::magnitude(diff).round() as u32;
    }
}
