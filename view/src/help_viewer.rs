use console_engine::{pixel, screen::Screen, Color};

pub fn help_screen(width: u32, height: u32) -> Screen {
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

    left_col.push(String::from("Apply Automata"));
    right_col.push(String::from("G"));

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

    screen.print_fbg(
        1,
        sc_y,
        "Fall Over or Game Over",
        Color::Rgb { r: 0, g: 255, b: 0 },
        Color::Black,
    );

    sc_y += 1;

    let title = "Balance Gradient: ";
    let ds = "Decrease Balance, ";
    let is = "Increase Balance";

    screen.print_fbg(
        1,
        sc_y,
        title,
        Color::Rgb {
            r: 255,
            g: 0,
            b: 255,
        },
        Color::Black,
    );

    screen.print_fbg(
        title.chars().count() as i32 + 1,
        sc_y,
        ds,
        Color::Rgb { r: 0, g: 0, b: 255 },
        Color::Black,
    );

    screen.print_fbg(
        title.chars().count() as i32 + ds.chars().count() as i32 + 2,
        sc_y,
        is,
        Color::Rgb { r: 255, g: 0, b: 0 },
        Color::Black,
    );

    sc_y += 1;

    screen.print_fbg(1, sc_y, "Esc to Return", Color::White, Color::Black);

    screen
}
