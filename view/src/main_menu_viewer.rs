use console_engine::{
    pixel,
    Color,
    screen::Screen
};

pub fn main_menu_screen(width: u32, height: u32) -> Screen {
    let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

    let mut left_col = Vec::new();
    let mut right_col = Vec::new();

    left_col.push(("Dangerous Deliveries", Color::Yellow));
    right_col.push("");

    left_col.push(("How to Play", Color::Green));
    right_col.push("Press 0");

    left_col.push(("Play", Color::Cyan));
    right_col.push("Press 1 or Esc");

    left_col.push(("Set Level", Color::Magenta));
    right_col.push("Press 2");

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
