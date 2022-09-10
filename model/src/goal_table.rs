use std::collections::HashSet;

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

        let mut region: (i32, i32) = (1, 0);
        for _ in 0..count {
            let p_x = (width as i32 / 2) + (region.0 * (width as i32 / 2)) - 2 * region.0;

            let p_y = (height as i32 / 2) + (region.1 * (height as i32 / 2)) - 2 * region.1;

            self.goals.insert((p_x, p_y));
            region = vec_ops::rotate_left(region);
        }
    }
}
