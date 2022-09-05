use std::collections::HashSet;
use rand::Rng;

use util::vec_ops;

pub struct GoalTable {
    goals: HashSet<(i32, i32)>,
}

impl GoalTable {
    pub fn new() -> Self {
        GoalTable {
            goals: HashSet::new(),
        }
    }
}

impl GoalTable {
    pub fn goals(&self) -> &HashSet<(i32, i32)> {
        &self.goals
    }

    pub fn add_goal(&mut self, goal: (i32, i32)) {
        self.goals.insert(goal);
    }

    pub fn count(&self) -> usize {
        self.goals.len() 
    }

    pub fn at_goal(&self, (x, y): (i32, i32)) -> bool {
        if let Some(_) = self.goals.get(&(x, y)) {
            return true;
        }
        return false;
    }

    pub fn remove_goal_if_reached(&mut self, (x, y): (i32, i32)) -> bool {
        self.goals.remove(&(x, y))
    }

    pub fn regen_goals(&mut self, width: u32, height: u32, count: u32) {
        self.goals.clear();

        let mut region = (1, 0);
        for _ in 0..count {
            let p_x = (width as i32 / 2) + 
                  (region.0 * (width as i32 / 4)) +
                  rand::thread_rng().gen_range((width as i32 / 8)..(width as i32 / 4 - 2)) as i32 * region.0.signum();

            let p_y = (height as i32 / 2) + 
                  (region.1 * (height as i32 / 4)) +
                  rand::thread_rng().gen_range((height as i32 / 8)..(height as i32 / 4 - 2)) as i32 * region.1.signum();

            // self.table[p_x as usize][p_y as usize] = Obstacle::Platform(self.get_height(p_x, p_y));

            self.goals.insert((p_x, p_y));
            for _ in 0..(rand::thread_rng().gen_range(1..=2) as u32) {
                region = vec_ops::rotate_left(region);
            }
        }
    }
}
