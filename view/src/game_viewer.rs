use console_engine::{
    pixel,
    screen::Screen,
    Color,
};

use std::collections::HashMap;
use std::cmp::max;

use model::util;

use model::cell_table::CellTable;
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

        gv.color_map.insert(Traversability::Flat, (Color::White, Color::Black));
        gv.color_map.insert(Traversability::Up, (Color::Cyan, Color::Black));
        gv.color_map.insert(Traversability::Down, (Color::Magenta, Color::Black));
        gv.color_map.insert(Traversability::No, (Color::DarkGrey, Color::Black));

        gv.symbol_map.insert(ObstacleType::Pit, '\u{25A1}');
        gv.symbol_map.insert(ObstacleType::Platform,  '\u{25A6}');

        // right
        gv.symbol_map.insert(ObstacleType::Rail(1, 0), '\u{21D2}');

        // left
        gv.symbol_map.insert(ObstacleType::Rail(-1, 0), '\u{21D0}');

        // up
        gv.symbol_map.insert(ObstacleType::Rail(0, 1), '\u{21D1}');

        // down
        gv.symbol_map.insert(ObstacleType::Rail(0, -1), '\u{21D3}');

        // diagonal right up
        gv.symbol_map.insert(ObstacleType::Rail(1, 1), '\u{21D7}');

        // diagonal left down
        gv.symbol_map.insert(ObstacleType::Rail(-1, -1), '\u{21D9}');

        // diagonal right down
        gv.symbol_map.insert(ObstacleType::Rail(1, -1), '\u{21D8}');

        // diagonal left up
        gv.symbol_map.insert(ObstacleType::Rail(-1, 1), '\u{21D6}');

        gv
    }
}

impl GameViewer {
    pub fn draw_layout(&self, table: &CellTable, player: &Player, fallover_threshold: f32, width: u32, height: u32) -> Screen {
        let r_panel_width = max(5, self.max_message_length + 2) as i32;
        let r_panel_x = width as i32 - r_panel_width - 1;
        let msg_log_tl_y = r_panel_width + 1;
        let msg_log_height = height as i32 - msg_log_tl_y;
        let table_view_width = width as i32 - r_panel_width;
        let table_view_height = height as i32;

        let table_view = self.draw_table(table, player, table_view_width as u32, table_view_height as u32);
        let balance_view = self.draw_balance(player, fallover_threshold, r_panel_width as u32);
        let msg_log_view = self.draw_msg_log(msg_log_height as u32);

        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        screen.print_screen(0, 0, &table_view);
        screen.print_screen(r_panel_x as i32, 0, &balance_view);
        screen.print_screen(r_panel_x as i32, msg_log_height as i32, &msg_log_view);

        let mut s = String::from("Distance Traveled: ");
        s.push_str(&player.distance_travled.to_string());
        
        // print distance traveled at top of the screen
        // TODO scoring
        screen.print((width as i32 / 2) - (s.chars().count() as i32 / 2), 
                     0, 
                     &s);

        screen
    }

    // return a Screen of dimensions width x height that maps a width x height section
    // of the CellTable centered on the player (any CellTable coordinates that are out of bounds
    // are clamped out and the screen doesn't draw anything there)
    pub fn draw_table(&self, table: &CellTable, player: &Player, width: u32, height: u32) -> Screen {
        let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

        // compute CellTable coordinates
        let middle = player.xy();
        let tl_x = (middle.0 - (width / 2) as i32).clamp(0, table.width() as i32 - 1);
        let tl_y = (middle.1 - (height / 2) as i32).clamp(0, table.height() as i32 - 1);
        let br_x = (middle.0 + (width / 2) as i32).clamp(0, table.width() as i32 - 1);
        let br_y = (middle.1 + (height / 2) as i32).clamp(0, table.height() as i32 - 1);

        // screen coords
        let mut sc_x = 0;
        let mut sc_y = 0;

        for x in tl_x..br_x {
            for y in tl_y..br_y {
                let obstacle_type = match table.get_obstacle(x, y) {
                    Obstacle::Platform(_) => ObstacleType::Platform,
                    Obstacle::Pit => ObstacleType::Pit,
                    Obstacle::Rail(_, dir) => {
                        let i_dir = util::discrete_jmp(dir);
                        ObstacleType::Rail(i_dir.0, i_dir.1)
                    },
                };

                let symbol = self.symbol_map[&obstacle_type];
                let colors = self.color_map[&table.traversability((player.x(), player.y()), (x, y))];

                screen.set_pxl(sc_x, sc_y, pixel::pxl_fbg(symbol, colors.0, colors.1));

                match obstacle_type {
                    ObstacleType::Pit => {},
                    _ => {
                            if x == player.x() && y == player.y() {
                                screen.set_pxl(sc_x, sc_y, pixel::pxl('@'));
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
        // create empty square
        let mut screen = Screen::new_fill(size, size, pixel::pxl(' '));

        // draw border
        screen.rect(0, 0, (size as i32) - 1, (size as i32) - 1, pixel::pxl('#'));

        // compute position of balance vector inside the rect
        let p_x = ((player.balance.0 / fallover_threshold) * (size as f32 / 2.0) ) as i32 + (size as i32 / 2);
        let p_y = ((player.balance.1 / fallover_threshold) * (size as f32 / 2.0) ) as i32 + (size as i32 / 2);

        // indicate balance with this symbol
        screen.set_pxl(p_x, p_y, pixel::pxl('*'));

        // return the screen so a ConsoleEngine can render it (wherever it wants)
        screen
    }

    pub fn draw_msg_log(&self, height: u32) -> Screen {
        let mut screen = Screen::new(self.max_message_length + 2, height);

        screen.rect(0, 0, self.max_message_length as i32 + 1, (height as i32) - 1, pixel::pxl('#'));

        let mut l_index = (self.message_log.len() as i32 - 1) as i32;
        let mut scr_x = 1;
        for y in (1..(screen.get_height() as i32 - 1)).rev() {
            if l_index < 0 {
                break;
            }

            for c in self.message_log[l_index as usize].chars() {
                if scr_x < self.max_message_length {
                    screen.set_pxl(scr_x as i32, y as i32, pixel::pxl(c));
                    scr_x += 1;
                }
                else {
                    break;
                }
            }

            l_index -= 1;
            scr_x = 0;
        }

        screen
    }

    pub fn add_message(&mut self, table: &CellTable, player: &Player, event: &PlayerEvent) {
        let mut message = String::new();
        match event {
            PlayerEvent::Move => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Moved onto platform."),
                    Obstacle::Pit => message.push_str("Game Over."),
                    Obstacle::Rail(_, _) => message.push_str("Grinding")
                }
            },
            PlayerEvent::Wait => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Waiting on platform."),
                    Obstacle::Pit => message.push_str("Game Over."),
                    Obstacle::Rail(_, _) => message.push_str("Grinding the rail.")
                }
            },
            PlayerEvent::FallOver => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Fell over onto platform."),
                    Obstacle::Pit => message.push_str("Fell into a pit. Game Over."),
                    Obstacle::Rail(_, _) => message.push_str("Fell over onto the rail (?)"),
                }
            },
            PlayerEvent::OffRail => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Got off the rail."),
                    Obstacle::Pit => message.push_str("Fell into a pit. Game Over."),
                    Obstacle::Rail(_, _) => message.push_str("Rail hop!"),
                }
            },
            PlayerEvent::OnRail => {
                match table.get_obstacle(player.x(), player.y()) {
                    Obstacle::Platform(_) => message.push_str("Grinding the platform (?)."),
                    Obstacle::Pit => message.push_str("Grinded(?) into a pit. Game Over."),
                    Obstacle::Rail(_, _) => message.push_str("You grind the rail."),
                }
            }

            PlayerEvent::GameOver => {
                message.push_str("Game Over");
            }
        }     

        self.message_log.push(message);

        if self.message_log.len() >= self.log_length {
            self.message_log.remove(0);
        }
    }
}