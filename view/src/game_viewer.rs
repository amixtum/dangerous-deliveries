use console_engine::{
    pixel,
    screen::Screen,
    Color,
};

use std::collections::HashMap;

use util::vec_ops;
use util::files;

use model::obstacle_table::ObstacleTable;
use model::goal_table::GoalTable;
use model::obstacle::Obstacle;
use model::obstacle::ObstacleType;
use model::traversability::Traversability;
use model::player::Player;
use model::player_event::PlayerEvent;

pub struct GameViewer {
    color_map: HashMap<Traversability, (Color, Color)>,
    symbol_map: HashMap<ObstacleType, char>,
    message_log: Vec<String>,
    log_length: usize,
    max_message_length: u32,
}

impl GameViewer {
    pub fn new(log_length: usize) -> Self {
        let mut gv = GameViewer {
            color_map: HashMap::new(),
            symbol_map: HashMap::new(),
            message_log: Vec::new(),
            log_length,
            max_message_length: 16,
        };

        gv.color_map.insert(Traversability::Flat, (Color::Blue, Color::Black));
        gv.color_map.insert(Traversability::Up, (Color::Magenta, Color::Black));
        gv.color_map.insert(Traversability::Down, (Color::Cyan, Color::Black));
        gv.color_map.insert(Traversability::No, (Color::Green, Color::Black));

        gv.symbol_map.insert(ObstacleType::Pit, 'x');
        gv.symbol_map.insert(ObstacleType::Platform,  '.');

        // bug (havent' found it yet)
        gv.symbol_map.insert(ObstacleType::Rail(0, 0), '.');

        // right
        gv.symbol_map.insert(ObstacleType::Rail(1, 0), '>');

        // left
        gv.symbol_map.insert(ObstacleType::Rail(-1, 0), '<');

        // up
        gv.symbol_map.insert(ObstacleType::Rail(0, -1), '^');

        // down
        gv.symbol_map.insert(ObstacleType::Rail(0, 1), 'v');

        // diagonal right up
        gv.symbol_map.insert(ObstacleType::Rail(1, -1), '/');

        // diagonal left down
        gv.symbol_map.insert(ObstacleType::Rail(-1, 1), 'd');

        // diagonal right down
        gv.symbol_map.insert(ObstacleType::Rail(1, 1), '\\');

        // diagonal left up
        gv.symbol_map.insert(ObstacleType::Rail(-1, -1), 'u');

        gv
    }
}

impl GameViewer {
    pub fn draw_layout(&self, table: &ObstacleTable, goals: &GoalTable, player: &Player, max_falls: u32, max_speed: f32, fallover_threshold: f32, width: u32, height: u32) -> Screen {
        let balance_size = 5;
        let speed_x = width as i32 - (balance_size * 2) - 1;
        let balance_x = speed_x - (balance_size * 2) - 1;
        let r_panel_width = self.max_message_length as i32 + 2;
        let r_panel_x = width as i32 - r_panel_width - 1;
        let msg_log_tl_y = balance_size;
        let msg_log_height = height as i32 - msg_log_tl_y - 2;
        let table_view_width = width as i32 - r_panel_width;
        let table_view_height = height as i32;

        let table_view = self.draw_table(table, goals, player, table_view_width as u32, table_view_height as u32);
        let balance_view = self.draw_balance(player, fallover_threshold, balance_size as u32);
        let speed_view = self.draw_speed(player, max_speed, balance_size as u32);
        let msg_log_view = self.draw_msg_log(msg_log_height as u32);

        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        screen.print_screen(0, 0, &table_view);
        screen.print_fbg(speed_x as i32, 0, "Speed", Color::Cyan, Color::Black);
        screen.print_screen(speed_x as i32, 1, &speed_view);
        screen.print_fbg(balance_x as i32, 0, "Balance", Color::Blue, Color::Black);
        screen.print_screen(balance_x as i32, 1, &balance_view);
        screen.print_screen(r_panel_x as i32, msg_log_tl_y as i32 + 1, &msg_log_view);

        let mut s = String::from("Time: ");
        s.push_str(&(player.time.round()).to_string());
        s.push_str(&format!(", Deliveries Left: {}, ", goals.count()));
        s.push_str(&format!("HP: {}, ", max_falls as i32 - player.n_falls));
        s.push_str("Help: press Esc");
        
        screen.print(0, height as i32 - 1, &s);

        screen
    }

    // return a Screen of dimensions width x height that maps a width x height section
    // of the ObstacleTable centered on the player (any ObstacleTable coordinates that are out of bounds
    // are clamped out and the screen doesn't draw anything there)
    pub fn draw_table(&self, table: &ObstacleTable, goals: &GoalTable, player: &Player, width: u32, height: u32) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        // compute ObstacleTable coordinates
        let middle = player.xy();
        let tl_x = (middle.0 - (width / 2) as i32).clamp(0, table.width() as i32 - 1);
        let tl_y = (middle.1 - (height / 2) as i32).clamp(0, table.height() as i32 - 1);
        let br_x = (middle.0 + (width / 2) as i32).clamp(0, table.width() as i32 - 1);
        let br_y = (middle.1 + (height / 2) as i32).clamp(0, table.height() as i32 - 1);

        // screen coords
        let mut sc_x = 0;
        let mut sc_y = 0;

        for x in tl_x..=br_x {
            for y in tl_y..=br_y {
                let obstacle_type = match table.get_obstacle(x, y) {
                    Obstacle::Platform(_) => ObstacleType::Platform,
                    Obstacle::Pit => ObstacleType::Pit,
                    Obstacle::Rail(_, dir) => {
                        let i_dir = vec_ops::discrete_jmp(dir);
                        ObstacleType::Rail(i_dir.0, i_dir.1)
                    },
                };

                let symbol = self.symbol_map[&obstacle_type];
                let colors = self.color_map[&table.traversability((player.x(), player.y()), (x, y))];

                screen.set_pxl(sc_x, sc_y, pixel::pxl_fbg(symbol, colors.0, colors.1));

                for goal in goals.goals() {
                    if x == goal.0 && y == goal.1 {
                        screen.set_pxl(sc_x, sc_y, pixel::pxl_fg('$', Color::Red));
                        break;
                    }
                }

                match obstacle_type {
                    ObstacleType::Pit => {},
                    _ => {
                            if x == player.x() && y == player.y() {
                                match player.recent_event {
                                    PlayerEvent::FallOver =>  {
                                        screen.set_pxl(sc_x, sc_y, pixel::pxl_fg('!', Color::Red));
                                    },
                                    _ => {
                                        screen.set_pxl(sc_x, sc_y, pixel::pxl('@'));
                                    }
                                }
                            }
                    }
                }

                
                sc_y += 1;
            }

            sc_y = 0;
            sc_x += 1;
        }


        screen
    }

    // returns a Screen which visualizes the direction of the Player's
    // Balance vector, and their closeness to falling over (the nearness of the indicator to the border)
    pub fn draw_balance(&self, player: &Player, fallover_threshold: f32, size: u32) -> Screen {
        self.draw_vector(player.balance, fallover_threshold, size, Color::Blue)
    }

    pub fn draw_speed(&self, player: &Player, max_speed: f32, size: u32) -> Screen {
        self.draw_vector(player.speed, max_speed, size, Color::Cyan)
    }

    pub fn draw_vector(&self, v: (f32, f32), max: f32, size: u32, color: Color) -> Screen {
        // create empty square
        let mut screen = Screen::new_fill(size * 2 + 1, size, pixel::pxl(' '));

        // draw border
        screen.rect(0, 0, size as i32 * 2, (size as i32) - 1, pixel::pxl_fg('#', color));

        // compute position of vector inside the rect
        let p_x = (((v.0 / max) * (size as f32 * 2.0)) as i32 + (size as i32)).clamp(0, size as i32 * 2);
        let p_y = (((v.1 / max) * (size as f32)) as i32 + (size as i32 / 2)).clamp(0, size as i32 - 1);

        // indicate speed with this symbol
        screen.set_pxl(p_x, p_y, pixel::pxl('*'));

        // return the screen so a ConsoleEngine can render it (wherever it wants)
        screen
    }

    pub fn draw_msg_log(&self, height: u32) -> Screen {
        let mut screen = Screen::new(self.max_message_length + 2, height);

        screen.rect(0, 0, self.max_message_length as i32 + 1, (height as i32) - 1, pixel::pxl('#'));

        let mut l_index = (self.message_log.len() as i32 - 1) as i32;
        let mut scr_y = height as i32 - 2;
        
        while scr_y > 0 && l_index >= 0 {
            if scr_y == height as i32 - 2 {
                screen.print(1, scr_y, &self.message_log[l_index as usize]);
            }
            else {
                screen.print_fbg(1, scr_y, &self.message_log[l_index as usize], Color::DarkGrey, Color::Black);
            }
            scr_y -= 1;
            l_index -= 1; 
        }

        screen
    }

    pub fn add_string(&mut self, s: String) {
        self.message_log.push(s);
        if self.message_log.len() > self.log_length {
            self.message_log.remove(0);
        }
    }

    pub fn add_message(&mut self, table: &ObstacleTable, player: &Player, event: &PlayerEvent) {
        /*
        let mut message = String::new();
        message.push_str("B: ");
        message.push_str(&player.balance.0.to_string());
        message.push_str(", ");
        message.push_str(&player.balance.1.to_string());

        self.message_log.push(message);

        if self.message_log.len() >= self.log_length {
            self.message_log.remove(0);
        }

        let mut message = String::new();
        message.push_str("S: ");
        message.push_str(&player.speed_x().to_string());
        message.push_str(", ");
        message.push_str(&player.speed_y().to_string());

        self.message_log.push(message); */

        if self.message_log.len() >= self.log_length {
            self.message_log.remove(0);
        }
        let mut message = String::new();
        match event {
            PlayerEvent::Move => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("On Platform"),
                    Obstacle::Pit => message.push_str("Game Over"),
                    Obstacle::Rail(_, _) => message.push_str("Grinding")
                }
            },
            PlayerEvent::Wait => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Waiting"),
                    Obstacle::Pit => message.push_str("Game Over"),
                    Obstacle::Rail(_, _) => message.push_str("Grinding")
                }
            },
            PlayerEvent::FallOver => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Fell over"),
                    Obstacle::Pit => message.push_str("Game Over"),
                    Obstacle::Rail(_, _) => message.push_str("Fell over"),
                }
            },
            PlayerEvent::OffRail => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Offrail"),
                    Obstacle::Pit => message.push_str("Game Over."),
                    Obstacle::Rail(_, _) => message.push_str("Rail hop!"),
                }
            },
            PlayerEvent::OnRail => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("On Platform"),
                    Obstacle::Pit => message.push_str("Game Over"),
                    Obstacle::Rail(_, _) => message.push_str("Grinding"),
                }
            }

            PlayerEvent::GameOver(_) => {
                message.push_str("Game Over");
            }
        }

        self.message_log.push(message);

        if self.message_log.len() >= self.log_length {
            self.message_log.remove(0);
        }
    }

    pub fn game_over_screen(&self, table: &ObstacleTable, goals: &GoalTable, player: &Player, max_goals: u32, width: u32, height: u32) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));
        screen.print_fbg((width as i32 / 2) - "Game Over".chars().count() as i32 / 2, (height as i32 / 2) - 1, "Game Over", Color::Red, Color::Black);

        let mut info = String::new();
        if let PlayerEvent::GameOver(time) = player.recent_event {
            info.push_str(&format!("Time: {}, Packages Delivered: {}", time, max_goals as i32 - goals.count() as i32));
        } 
        else {
            info.push_str(&format!("Packages Delivered: {}", max_goals as i32 - goals.count() as i32));
        }
        screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, height as i32 / 2, &info);

        info.clear(); 

        info.push_str("Press R to restart. Press Esc to exit.");
        screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, (height as i32 / 2) + 1, &info);

        screen
    }

    pub fn win_screen(&self, player: &Player, width: u32, height: u32) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));
        screen.print_fbg((width as i32 / 2) - "You Win".chars().count() as i32 / 2, (height as i32 / 2) - 1, "You Win", Color::Green, Color::Black);

        let mut info = String::new();
        if let PlayerEvent::GameOver(time) = player.recent_event {
            info.push_str(&format!("Time: {}", time));
            screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, height as i32 / 2, &info);
        } 

        info.clear(); 

        info.push_str("Press R to restart. Press Esc to exit.");
        screen.print((width as i32 / 2) - info.chars().count() as i32 / 2, (height as i32 / 2) + 1, &info);

        screen
    }

    pub fn help_screen(&self, width: u32, height: u32) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        let mut left_col = Vec::new();
        let mut right_col = Vec::new();

        left_col.push(String::from("Look Mode"));
        right_col.push(String::from("Semicolon"));

        left_col.push(String::from("Left"));
        right_col.push(String::from("A or H"));

        left_col.push(String::from("Right"));
        right_col.push(String::from("D or L"));

        left_col.push(String::from("Up"));
        right_col.push(String::from("W or K"));

        left_col.push(String::from("Down"));
        right_col.push(String::from("S or J"));

        left_col.push(String::from("NorthEast"));
        right_col.push(String::from("E or U"));

        left_col.push(String::from("NorthWest"));
        right_col.push(String::from("Q or Y"));

        left_col.push(String::from("SouthEast"));
        right_col.push(String::from("C or N"));

        left_col.push(String::from("SouthWest"));
        right_col.push(String::from("Z or B"));

        left_col.push(String::from("Wait"));
        right_col.push(String::from("Tab or Period"));

        left_col.push(String::from("Restart"));
        right_col.push(String::from("Enter"));

        left_col.push(String::from("Menu"));
        right_col.push(String::from("Esc"));

        left_col.push(String::from("Exit Game"));
        right_col.push(String::from("Ctrl+C"));

        let mut col = 0;
        let mut sc_y = 0;

        while col < left_col.len() && col < right_col.len() {
            screen.print(1, sc_y, &left_col[col]);
            screen.print(width as i32 / 2, sc_y, &right_col[col]);
            sc_y += 1;
            col += 1;
        }

        screen.print_fbg(1, sc_y, "Color Coding", Color::Yellow, Color::Black);

        sc_y += 1;

        screen.print_fbg(1, sc_y, "Not Traversable", Color::Green, Color::Black);

        sc_y += 1;

        screen.print_fbg(1, sc_y, "Same level", Color::Blue, Color::Black);

        sc_y += 1;

        screen.print_fbg(1, sc_y, "Down one level", Color::Cyan, Color::Black);

        sc_y += 1;
        
        screen.print_fbg(1, sc_y, "Up one level", Color::Magenta, Color::Black);

        screen
    }

    pub fn file_chooser(&self, width: u32, height: u32, starts_with: &str) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        let files = files::get_config_filenames(starts_with);

        let mut sc_y = 0;
        let mut index = 0;
        for filename in files {
            let number = format!("{} : ", index);
            screen.print(1, sc_y, &number);
            screen.print(number.chars().count() as i32 + 1, sc_y, &filename);
            sc_y += 1;
            index += 1;
        }

        screen
    }

    pub fn draw_main_menu(&self, width: u32, height: u32) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        let mut left_col = Vec::new();
        let mut right_col = Vec::new();

        left_col.push(("Dangerous Deliveries", Color::Yellow));
        right_col.push("");

        left_col.push(("How to Play", Color::Green));
        right_col.push("Press 0");

        left_col.push(("Play", Color::Cyan));
        right_col.push("Press 1 or Esc");

        left_col.push(("Exit", Color::Red));
        right_col.push("Press Q or Ctrl+C");

        let mut index = 0;
        let mut sc_y = 0;

        while index < left_col.len() {
            if index == 0 {
                screen.print_fbg(width as i32 / 4, sc_y + 1, &left_col[index].0, left_col[index].1, Color::Black);
            }
            else {
                screen.print_fbg(1, sc_y, &left_col[index].0, left_col[index].1, Color::Black);
                screen.print_fbg(width as i32 / 2, sc_y, &right_col[index], left_col[index].1, Color::Black);
            }

            index += 1;
            sc_y += height as i32 / left_col.len() as i32;
        } 

        screen
    }

    pub fn clear_log(&mut self) {
        self.message_log.clear();
    }
}
