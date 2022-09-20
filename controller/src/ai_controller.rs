use super::player_controller::PlayerController;

use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;

use model::visibility;
use rltk::{RandomNumberGenerator, Point, DistanceAlg};
use util::vec_ops;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Pos(i32, i32);

pub struct AIController {
    pub player: Player,
    pub goal: (i32, i32),
}

impl AIController {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        AIController {
            player: Player::new(start_x, start_y),
            goal: (-1, -1),
        }
    }
}

impl AIController {
    pub fn reset(&mut self, _obs_table: &ObstacleTable, x: i32, y: i32) {
        self.player.position.0 = x;
        self.player.position.1 = y;
        self.player.balance = (0.0, 0.0);
        self.player.speed = (0.0, 0.0);
        self.player.n_falls = 0;
        self.player.recent_event = PlayerEvent::GameOver(self.player.time.round() as i32);
        self.goal = (-1, -1);
    }

    pub fn set_goal(&mut self, pos: (i32, i32)) {
        self.goal = pos;
    }

    pub fn choose_goal(&mut self, obs_table: &ObstacleTable, sight_radius: u32) {
        let mut rng = RandomNumberGenerator::new();
        let ai_player = &self.player;
        let visible = visibility::get_visible(ai_player.position, obs_table, sight_radius * 2);
        let visible = visible.iter().filter(|p| {
            obs_table.get_obstacle(p.0, p.1) == Obstacle::Platform
        }).collect::<Vec<_>>();
        let potential_goals = visible.iter().filter(|p| {
            (rltk::DistanceAlg::Pythagoras.distance2d(Point::new(ai_player.x(), ai_player.y()), Point::new(p.0, p.1)) >= sight_radius as f32) &&
            obs_table.get_obstacle(p.0, p.1) == Obstacle::Platform
        }).collect::<Vec<_>>();

        if let Some(goal) = rng.random_slice_entry(&potential_goals) {
            self.set_goal(***goal);
        }
        else if let Some(goal) = rng.random_slice_entry(&visible) {
            self.set_goal(**goal);
        } else {
            self.set_goal((obs_table.width() as i32 / 2, obs_table.height() as i32 / 2));
        }
    }

    pub fn move_player(&mut self, obs_table: &ObstacleTable, player_control: &PlayerController) {
        if self.goal.0 == -1 || self.goal.1 == -1 {
            return;
        }
        self.player = self.next_move(obs_table, player_control);
    }

    pub fn reached_goal(&self, radius: f32) -> bool {
        DistanceAlg::Pythagoras.distance2d(Point::new(self.player.x(), self.player.y()), Point::new(self.goal.0, self.goal.1)) <= radius
    }

    pub fn next_move(
        &mut self,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Player {
        //let mut rng = RandomNumberGenerator::new();
        let try_platform = self.next_platform(obs_table, player_control);

        // if the only platform is the one the player is standing on,
        // try again, this time including rails
        // dont' exclude the possibility of staying in place indefinitely
        if try_platform.x() == self.player.x() && try_platform.y() == self.player.y() {
            let mut moves = self.get_moves(&self.player, obs_table, player_control);

            if moves.len() > 0 {
                moves.sort_by(|l, r| self.dist_to_goal(&l.0).cmp(&self.dist_to_goal(&r.0)));
                if moves.len() == 1 {
                    return moves[0].0;
                } else if moves.len() > 1 {
                    match moves[1].0.recent_event {
                        PlayerEvent::FallOver |
                        PlayerEvent::GameOver(_) => {
                            return moves[0].0;
                        }
                        _ => {
                            return moves[1].0;
                        }
                    }
                }
            }
        }

        return try_platform;
    }

    pub fn next_platform(
        &mut self,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Player {
        let mut moves = self.get_moves_platform(&self.player, obs_table, player_control);

        if moves.len() > 0 {
            moves.sort_by(|l, r| self.dist_to_goal(&l.0).cmp(&self.dist_to_goal(&r.0)));

            if moves.len() == 1 {
                return moves[0].0;
            } else if moves.len() > 1 {
                match moves[1].0.recent_event {
                    PlayerEvent::FallOver |
                    PlayerEvent::GameOver(_) => {
                        return moves[0].0;
                    }
                    _ => {
                        return moves[1].0;
                    }
                }
            }
        }

        self.player.recent_event = PlayerEvent::GameOver(self.player.time.round() as i32);

        return self.player;
    }

    pub fn get_moves(
        &self,
        player: &Player,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Vec<(Player, f32)> {
        let mut moves = Vec::new();
        let mut falls: Vec<(Player, f32)> = Vec::new();
        let mut rng = RandomNumberGenerator::new();

        // iterate through all possible inputs to the player controller
        // and push the new player with the time the move took
        for key in player_control.get_keys() {
            let mov = player_control.move_player(&obs_table, &player, key);

            match mov.recent_event {
                PlayerEvent::GameOver(_) => {}
                PlayerEvent::FallOver => {
                    falls.push((mov, 999.0));
                }
                _ => match obs_table.get_obstacle(mov.x(), mov.y()) {
                    Obstacle::Wall |
                    Obstacle::Pit => {}
                    _ => {
                        moves.push((mov, DistanceAlg::Pythagoras.distance2d(Point::new(mov.x(), mov.y()), Point::new(self.goal.0, self.goal.1))));
                    }
                },
            }
        }

        if moves.len() == 0 {
            if let Some(choice) = rng.random_slice_entry(&falls) {
                moves.push(*choice);
            }
        }

        moves
    }

    pub fn get_moves_platform(
        &self,
        player: &Player,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Vec<(Player, f32)> {
        let mut rng = RandomNumberGenerator::new();
        let mut moves = Vec::new();
        let mut falls = Vec::new();

        // iterate through the possible inputs and push any moves that end on a platform
        // along with the time it took to make the move
        for key in player_control.get_keys() {
            let mov = player_control.move_player(&obs_table, &player, key);
            match mov.recent_event {
                PlayerEvent::GameOver(_) => {}
                PlayerEvent::FallOver => {
                    falls.push((mov, DistanceAlg::Pythagoras.distance2d(Point::new(mov.x(), mov.y()), Point::new(self.goal.0, self.goal.1))));
                }
                _ => match obs_table.get_obstacle(mov.x(), mov.y()) {
                    Obstacle::Platform => {
                        moves.push((mov, DistanceAlg::Pythagoras.distance2d(Point::new(mov.x(), mov.y()), Point::new(self.goal.0, self.goal.1))));
                    }
                    _ => {}
                },
            }
        }

        if moves.len() == 0 {
            if let Some(choice) = rng.random_slice_entry(&falls) {
                moves.push(*choice);
            }
        }

        moves
    }

    pub fn dist_to_goal(&self, player: &Player) -> u32 {
        let diff = (
            (self.goal.0 - player.x()) as f32,
            (self.goal.1 - player.y()) as f32,
        );
        return vec_ops::magnitude(diff).round() as u32;
    }
}
