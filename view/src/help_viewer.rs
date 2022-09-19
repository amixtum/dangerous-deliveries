use rltk::RGB;

pub fn help_screen(ctx: &mut rltk::Rltk, width: u32, _height: u32) {
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
        ctx.print(1, sc_y, &left_col[col]);
        ctx.print(width as i32 / 2, sc_y, &right_col[col]);
        sc_y += 1;
        col += 1;
    }


    ctx.print_color(1, sc_y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Color Coding");

    sc_y += 1;

    ctx.print_color(
        1,
        sc_y,
        RGB::from_u8(0,255,0),
        RGB::named(rltk::BLACK),
        "Fall Over or Game Over",
    );

    sc_y += 1;

    let title = "Balance Gradient: ";
    let ds = "Decrease Balance, ";
    let is = "Increase Balance";

    ctx.print_color(
        1,
        sc_y,
        RGB::from_u8(255, 0, 255),
        RGB::named(rltk::BLACK),
        title,
    );

    ctx.print_color(
    title.chars().count() as i32 + 1,
        sc_y,
        RGB::from_u8(0, 0, 255),
        RGB::named(rltk::BLACK),
        ds,
    );

    ctx.print_color(
        title.chars().count() as i32 + ds.chars().count() as i32 + 2,
        sc_y,
        RGB::from_u8(255, 0, 0),
        RGB::named(rltk::BLACK),
        is,
    );

    sc_y += 1;

    ctx.print(1, sc_y, "Esc to Return");
}
