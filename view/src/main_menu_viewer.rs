use rltk::RGB;

pub fn main_menu_screen(ctx: &mut rltk::Rltk, width: u32, height: u32) {
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();

    left_col.push(("Dangerous Deliveries", RGB::named(rltk::YELLOW)));
    right_col.push("");

    left_col.push(("How to Play", RGB::named(rltk::GREEN)));
    right_col.push("Press 0");

    left_col.push(("Play", RGB::named(rltk::CYAN)));
    right_col.push("Press 1 or Esc");

    //left_col.push(("Set Level", Color::Magenta));
    //right_col.push("Press 2");

    left_col.push(("Exit", RGB::named(rltk::RED)));
    right_col.push("Press Q or Ctrl+C");

    let mut index = 0;
    let mut sc_y = 0;

    while index < left_col.len() {
        if index == 0 {
            ctx.print_color(
                width as i32 / 4, 
                sc_y + 1, 
                left_col[index].1, 
                RGB::named(rltk::BLACK), 
                &left_col[index].0);
        } else {
            ctx.print_color(
                1, 
                sc_y, 
                left_col[index].1, 
                RGB::named(rltk::BLACK), 
                &left_col[index].0);
            ctx.print_color(
                width as i32 / 2, 
                sc_y, 
                left_col[index].1, 
                RGB::named(rltk::BLACK), 
                &right_col[index]);
        }

        index += 1;
        sc_y += height as i32 / left_col.len() as i32;
    }
}
