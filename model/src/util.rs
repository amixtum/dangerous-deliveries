pub fn neighbors((x, y): (usize, usize), 
                 (tl_x, tl_y): (usize, usize), 
                 (br_x, br_y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = Vec::new();

    if x > tl_x {
        v.push((x - 1, y));

        if y > tl_y {
            v.push((x - 1, y - 1));
        }

        if y < br_y {
            v.push((x - 1, y + 1));
        }
    }

    if x < br_x {
        v.push((x + 1, y));

        if y > tl_y {
            v.push((x + 1, y - 1));
        }

        if y < br_y {
            v.push((x + 1, y + 1));
        }
    }

    if y > tl_y {
        v.push((x, y - 1));
    }

    if y < br_y {
        v.push((x, y + 1));
    }

    v
}

pub fn magnitude((x, y): (f32, f32)) -> f32 {
    return ((x * x) + (y * y)).sqrt();
}
