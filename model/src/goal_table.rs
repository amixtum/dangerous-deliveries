use std::{collections::{HashMap}};

use rltk::RGB;

pub struct GoalTable {
    pub goals: HashMap<(i32, i32), (usize, RGB)>,
    index_map: HashMap<usize, (i32, i32)>,
}

impl GoalTable {
    pub fn new() -> Self {
        GoalTable {
            goals: HashMap::new(),
            index_map: HashMap::new(),
        }
    }
}

impl GoalTable {
    pub fn add_goal(&mut self, goal: (i32, i32), dest: (usize, RGB)) {
        self.goals.insert(goal, dest);
        self.index_map.insert(dest.0, goal);
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

    pub fn remove_goal_index(&mut self, index: usize) -> bool {
        if let Some(entry) = self.index_map.remove(&index) {
            match self.goals.remove(&entry) {
                    None => return false,
                    Some(_) => return true,
            }
        }
        false
    }

    pub fn remove_goal_if_reached(&mut self, (x, y): (i32, i32)) -> bool {
        match self.goals.remove(&(x, y)) {
            None => false,
            Some(entry) => match self.index_map.remove(&entry.0) {
                None => false,
                Some(_) => true,
            }
        }
    }
}
