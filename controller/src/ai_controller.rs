use super::player_controller::PlayerController;

use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;

use rltk::{Algorithm2D, DistanceAlg, Point, RandomNumberGenerator, NavigationPath};
use util::vec_ops;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Pos(i32, i32);

pub struct AIController {
    pub player: Player,
    pub goal: (i32, i32),
    path: NavigationPath,
    path_idx: usize,
    rng: RandomNumberGenerator,
}

impl AIController {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        AIController {
            player: Player::new(start_x, start_y),
            goal: (-1, -1),
            path: NavigationPath::new(),
            path_idx: 1,
            rng: RandomNumberGenerator::new(),
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

    pub fn choose_goal(&mut self, obs_table: &ObstacleTable, _sight_radius: u32) {
        //let ai_player = &self.player;
        //let norm_speed = vec_ops::normalize(ai_player.speed);

        let center = self.rng.random_slice_entry(&obs_table.platforms); 
        /*if !f32::is_nan(norm_speed.0) {
            center = (ai_player.x() + (norm_speed.0 * sight_radius as f32) as i32, ai_player.y() + (norm_speed.1 * sight_radius as f32) as i32);
        }*/
        if let Some(center) = center {
            self.set_goal(*center);
        }
        else {
            self.set_goal((obs_table.width() as i32 / 2, obs_table.height() as i32 / 2));
        }

        self.path = rltk::a_star_search(
            obs_table.point2d_to_index(Point::new(self.player.x(), self.player.y())),
            obs_table.point2d_to_index(Point::new(self.goal.0, self.goal.1)),
            obs_table,
        );

        self.path_idx = 1;
    }

    pub fn move_player(&mut self, obs_table: &ObstacleTable, player_control: &PlayerController) {
        if self.goal.0 == -1 || self.goal.1 == -1 {
            return;
        }

        self.player = self.next_move(obs_table, player_control);
    }

    pub fn reached_goal(&self, radius: f32) -> bool {
        DistanceAlg::Manhattan.distance2d(
            Point::new(self.player.x(), self.player.y()),
            Point::new(self.goal.0, self.goal.1),
        ) <= radius
    }

    pub fn next_move(
        &mut self,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Player {
        let clone = Player::clone(&self.player);
        let mut moves = self.get_moves(&clone, obs_table, player_control);
        let next_step = obs_table.index_to_point2d(self.path.steps[self.path_idx]);

        if moves.len() == 0 {
            return self.player;
        }


        if moves.len() == 1 {
            if moves[0].0.x() == next_step.x && moves[0].0.y() == next_step.y {
                self.path_idx += 1;
                if self.path_idx >= self.path.steps.len() {
                    self.path_idx = 1;
                }
            }
            return moves[0].0;
        }

        moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        
        for index in 0..moves.len() {
            if moves[index].0.x() != self.player.x() || moves[index].0.y() != self.player.y() {
                if moves[index].0.x() == next_step.x && moves[index].0.y() == next_step.y {
                    self.path_idx += 1;
                    if self.path_idx >= self.path.steps.len() {
                        self.path_idx = 1;
                    }
                }

                return moves[index].0;
            }
            else {
                if self.rng.roll_dice(1, 3) == 1 {
                    if moves[index].0.x() == next_step.x && moves[index].0.y() == next_step.y {
                        self.path_idx += 1;
                        if self.path_idx >= self.path.steps.len() {
                            self.path_idx = 1;
                        }
                    }
                    return moves[index].0;
                }
            }
        }

        if moves[0].0.x() == next_step.x && moves[0].0.y() == next_step.y {
            self.path_idx += 1;
            if self.path_idx >= self.path.steps.len() {
                self.path_idx = 1;
            }
        }

        return moves[0].0;
    }

    pub fn get_moves(
        &mut self,
        player: &Player,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Vec<(Player, f32)> {
        let mut moves = Vec::new();
        let mut falls: Vec<(Player, f32)> = Vec::new();

        // iterate through all possible inputs to the player controller
        // and push the new player that is closest to the next
        // step in this controller's path (computed in choose_goal)
        for key in player_control.get_keys() {
            let mov = player_control.move_player(&obs_table, &player, key);

            match mov.recent_event {
                PlayerEvent::GameOver(_) | PlayerEvent::Respawn => {}
                PlayerEvent::FallOver => {
                    falls.push((mov, 1024.0));
                }
                _ => match obs_table.get_obstacle(mov.x(), mov.y()) {
                    Obstacle::Wall | Obstacle::Pit => {}
                    _ => {
                        if self.path.success && self.path_idx < self.path.steps.len() {
                            let step = obs_table.index_to_point2d(self.path.steps[self.path_idx]);
                            moves.push((
                                mov,
                                DistanceAlg::Manhattan
                                    .distance2d(step, Point::new(mov.x(), mov.y())),
                            ));
                        }
                    }
                },
            }
        }

        if moves.len() == 0 {
            if let Some(choice) = self.rng.random_slice_entry(&falls) {
                moves.push(*choice);
            }
        }

        moves
    }

    pub fn get_moves_platform(
        &mut self,
        player: &Player,
        obs_table: &ObstacleTable,
        player_control: &PlayerController,
    ) -> Vec<(Player, f32)> {
        let mut moves = Vec::new();
        let mut falls = Vec::new();

        // iterate through the possible inputs and push any moves that end on a platform
        // along with the time it took to make the move
        for key in player_control.get_keys() {
            let mov = player_control.move_player(&obs_table, &player, key);
            match mov.recent_event {
                PlayerEvent::GameOver(_) | PlayerEvent::Respawn => {}
                PlayerEvent::FallOver => {
                    falls.push((mov, 99999.0));
                }
                _ => match obs_table.get_obstacle(mov.x(), mov.y()) {
                    Obstacle::Platform => {
                        moves.push((
                            mov,
                            DistanceAlg::Manhattan.distance2d(
                                Point::new(self.player.x(), self.player.y()),
                                Point::new(mov.x(), mov.y()),
                            ),
                        ));
                    }
                    _ => {}
                },
            }
        }

        if moves.len() == 0 {
            if let Some(choice) = self.rng.random_slice_entry(&falls) {
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
