use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alphabet {
    Fwd,    // f
    Left,   // l
    Right,  // r
    Up,     // u
    Down,   // d
    Save,   // [
    Return, // ]
    Place,  // p
    None,
}

impl Alphabet {
    pub fn to_char(a: Alphabet) -> char {
        match a {
            Alphabet::Fwd => 'f',
            Alphabet::Left => 'l',
            Alphabet::Right => 'r',
            Alphabet::Up => 'u',
            Alphabet::Down => 'd',
            Alphabet::Save => '[',
            Alphabet::Return => ']',
            Alphabet::Place => 'p',
            Alphabet::None => '0',
        }
    }

    pub fn from_char(c: char) -> Option<Alphabet> {
        if c == 'f' {
            return Some(Alphabet::Fwd);
        } else if c == 'l' {
            return Some(Alphabet::Left);
        } else if c == 'r' {
            return Some(Alphabet::Right);
        } else if c == 'u' {
            return Some(Alphabet::Up);
        } else if c == 'd' {
            return Some(Alphabet::Down);
        } else if c == '[' {
            return Some(Alphabet::Save);
        } else if c == ']' {
            return Some(Alphabet::Return);
        } else if c == 'p' {
            return Some(Alphabet::Place);
        } else if c == '0' {
            return Some(Alphabet::None);
        }

        None
    }
}

pub struct LSystem {
    current: Vec<Alphabet>,
    rules: HashMap<(Alphabet, Alphabet, Alphabet), Vec<Alphabet>>,

    pub iterations: u32,
    pub turtles: u32,
}

pub struct Turtle {
    pub position: (i32, i32, i32),
    pub direction: (i32, i32, i32),
}

impl Turtle {
    pub fn new(position: (i32, i32, i32), direction: (i32, i32, i32)) -> Self {
        Turtle {
            position,
            direction,
        }
    }
}

impl LSystem {
    pub fn new() -> Self {
        LSystem {
            current: Vec::new(),
            rules: HashMap::new(),
            iterations: 2,
            turtles: 1,
        }
    }

    pub fn from_file(filepath: &str) -> Self {
        let mut lsystem = LSystem::new();
        if let Ok(contents) = fs::read_to_string(filepath) {
            let mut axiom = Vec::new();
            let mut first_line = true;
            for line in contents.lines() {
                if let Some(c) = line.chars().nth(0) {
                    if c == '#' {
                        continue;
                    }
                } else {
                    continue;
                }

                let words: Vec<&str> = line.split_ascii_whitespace().collect();
                if words[0] == "iterations" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        lsystem.iterations = num;
                    }
                    continue;
                } else if words[0] == "turtles" {
                    if let Ok(num) = words[1].parse::<u32>() {
                        lsystem.turtles = num;
                    }
                    continue;
                } else if first_line {
                    for s in line.split_ascii_whitespace() {
                        if let Some(c) = s.chars().nth(0) {
                            if let Some(letter) = Alphabet::from_char(c) {
                                axiom.push(letter);
                            }
                        }
                    }
                    first_line = false;
                } else {
                    let mut pred = Alphabet::None;
                    let mut target = Alphabet::None;
                    let mut succ = Alphabet::None;
                    let mut exp = Vec::new();

                    let mut counter = 0;
                    for s in line.split_ascii_whitespace() {
                        if counter == 0 {
                            if let Some(c) = s.chars().nth(0) {
                                if let Some(letter) = Alphabet::from_char(c) {
                                    pred = letter;
                                }
                            }

                            counter += 1;
                        } else if counter == 1 {
                            if let Some(c) = s.chars().nth(0) {
                                if let Some(letter) = Alphabet::from_char(c) {
                                    target = letter;
                                }
                            }

                            counter += 1;
                        } else if counter == 2 {
                            if let Some(c) = s.chars().nth(0) {
                                if let Some(letter) = Alphabet::from_char(c) {
                                    succ = letter;
                                }
                            }

                            counter += 1;
                        } else {
                            if let Some(c) = s.chars().nth(0) {
                                if let Some(letter) = Alphabet::from_char(c) {
                                    exp.push(letter);
                                }
                            }
                        }
                    }

                    lsystem.add_rule((pred, target, succ), exp);
                }
            }

            lsystem.set_current(axiom);
        }

        lsystem
    }
}

impl LSystem {
    pub fn add_rule(
        &mut self,
        (pred, target, succ): (Alphabet, Alphabet, Alphabet),
        exp: Vec<Alphabet>,
    ) {
        self.rules.insert((pred, target, succ), exp);
    }

    pub fn get_exp(
        &self,
        (pred, target, succ): (Alphabet, Alphabet, Alphabet),
    ) -> Option<&Vec<Alphabet>> {
        self.rules.get(&(pred, target, succ))
    }

    pub fn get_current(&self) -> &Vec<Alphabet> {
        &self.current
    }

    pub fn set_current(&mut self, current: Vec<Alphabet>) {
        self.current = current;
    }

    pub fn update_n(&mut self, n: u32) {
        for _ in 0..n {
            self.update_all();
        }
    }

    pub fn update_all(&mut self) {
        let mut updated = Vec::new();
        let mut index = 0;

        while index < self.current.len() {
            let mut pred = Alphabet::None;
            if index > 0 {
                pred = self.current[index - 1];
            }

            let target = self.current[index];

            let mut succ = Alphabet::None;
            if index < self.current.len() - 1 {
                succ = self.current[index + 1];
            }

            if let Some(exp) = self.get_exp((pred, target, succ)) {
                let mut exp_index = 0;
                while exp_index < exp.len() {
                    updated.push(exp[exp_index]);
                    exp_index += 1;
                }
            } else if let Some(exp) = self.get_exp((Alphabet::None, target, succ)) {
                let mut exp_index = 0;
                while exp_index < exp.len() {
                    updated.push(exp[exp_index]);
                    exp_index += 1;
                }
            } else if let Some(exp) = self.get_exp((pred, target, Alphabet::None)) {
                let mut exp_index = 0;
                while exp_index < exp.len() {
                    updated.push(exp[exp_index]);
                    exp_index += 1;
                }
            } else if let Some(exp) = self.get_exp((Alphabet::None, target, Alphabet::None)) {
                let mut exp_index = 0;
                while exp_index < exp.len() {
                    updated.push(exp[exp_index]);
                    exp_index += 1;
                }
            } else {
                updated.push(self.current[index]);
            }
            index += 1;
        }

        self.current = updated;
    }
}

/*
let obstacle_type = match table.get_obstacle(x, y) {
    Obstacle::Platform(_) => ObstacleType::Platform,
    Obstacle::Pit => ObstacleType::Pit,
    Obstacle::Rail(_, dir) => {
        let i_dir = vec_ops::discrete_jmp(dir);
        ObstacleType::Rail(i_dir.0, i_dir.1)
    },
};
*/
