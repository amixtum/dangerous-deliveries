use console_engine::{pixel, screen::Screen, Color};

pub fn size_chooser_viewer(width: u32, height: u32) -> Screen {
    let mut screen = Screen::new_fill(width, height, pixel::pxl(' '));

    let mut left_col = Vec::new();
    let mut right_col = Vec::new();

    left_col.push(("Small: 40 x 20", Color::Green));
    right_col.push("Press 0");

    left_col.push(("Medium: 80 x 40", Color::Cyan));
    right_col.push("Press 1");

    left_col.push(("Back", Color::White));
    right_col.push("Press Esc");

    let mut index = 0;
    let mut sc_y = 0;

    while index < left_col.len() {
        screen.print_fbg(1, sc_y, &left_col[index].0, left_col[index].1, Color::Black);
        screen.print_fbg(
            width as i32 / 2,
            sc_y,
            &right_col[index],
            left_col[index].1,
            Color::Black,
        );

        index += 1;
        sc_y += height as i32 / left_col.len() as i32;
    }

    screen
}
