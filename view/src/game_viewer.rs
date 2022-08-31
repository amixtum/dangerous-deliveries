use console_engine::{
    ConsoleEngine,
    pixel::{self, Pixel},
    screen::Screen,
    Color,
};

use std::collections::HashMap;


use super::direction::Direction;

use model::util;

use model::cell_table::CellTable;
use model::obstacle::ObstacleType;
use model::traversability::Traversability;
use model::player::Player;

pub struct GameViewer {
    color_map: HashMap<Traversability, (Color, Color)>,
    symbol_map: HashMap<ObstacleType, char>,

    last_balance_index: (i32, i32),
}

impl GameViewer {
    pub fn new() -> Self {
        let mut gv = GameViewer {
            color_map: HashMap::new(),
            symbol_map: HashMap::new(),
            last_balance_index: (-1, -1),
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
    pub fn draw_table(&self, table: &CellTable, player: &Player, width: u32, height: u32) -> Screen {
        
    }

    pub fn draw_balance(&self, player: &Player, fallover_threshold: f32, size: u32) -> Screen {
        // create empty square
        let mut screen = Screen::new_fill(size as u32, size as u32, pixel::pxl(' '));

        // draw border
        screen.rect(0, 0, size as i32, size as i32, pixel::pxl('#'));

        // compute position of balance vector inside the rect
        let p_x = ((player.balance.0 / fallover_threshold) * (size as f32 / 2.0) ) as i32 + (size as i32 / 2);
        let p_y = ((player.balance.1 / fallover_threshold) * (size as f32 / 2.0) ) as i32 + (size as i32 / 2);

        // indicate balance with this symbol
        screen.set_pxl(p_x, p_y, pixel::pxl('*'));

        // return the screen so a ConsoleEngine can render it (wherever it wants)
        screen
    }
}
