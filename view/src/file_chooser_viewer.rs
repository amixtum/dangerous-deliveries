use console_engine::{
    pixel,
    screen::Screen
};

use util::files;

pub fn file_chooser_screen(width: u32, height: u32, starts_with: &str) -> Screen {
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