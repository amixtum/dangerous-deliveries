use std::collections::HashSet;

use model::{obstacle::Obstacle, obstacle_table::ObstacleTable, player::Player};
use util::vec_ops;

use crate::ai_controller::AIController;

pub fn update_blocked(
    table: &mut ObstacleTable,
    human: &Player,
    ai: &Vec<AIController>,
    dead: &HashSet<u32>,
) {
    table.blocked.clear();
    table.blocked.insert(human.position, Player::clone(human));
    for p in ai.iter().enumerate() {
        if !dead.contains(&(p.0 as u32)) {
            table
                .blocked
                .insert(p.1.player.position, Player::clone(&p.1.player));
        }
    }
}

pub fn collide(table: &ObstacleTable, slider: &Player, collided: &Player) -> Player {
    let mut slide = Player::clone(&slider);
    let new_speed = (
        slider.speed_x() + collided.speed_x(),
        slider.speed_y() + collided.speed_y(),
    );
    slide.speed = new_speed;
    let try_x = ((slide.x() as f32 + new_speed.0) as i32).clamp(0, table.width() as i32 - 1);
    let try_y = ((slide.y() as f32 + new_speed.1) as i32).clamp(0, table.height() as i32 - 1);
    if table.get_obstacle(try_x, try_y) == Obstacle::Platform
        && !table.blocked.contains_key(&(try_x, try_y))
    {
        slide.position = (try_x, try_y);
    } else {
        let mut nbrs = vec_ops::neighbors(
            slide.position,
            (0, 0),
            (table.width() as i32 - 1, table.height() as i32 - 1),
        );
        nbrs.sort_by(|l, r| {
            vec_ops::magnitude((l.0 as f32 - try_x as f32, l.1 as f32 - try_y as f32))
                .partial_cmp(&vec_ops::magnitude((
                    r.0 as f32 - try_x as f32,
                    r.1 as f32 - try_y as f32,
                )))
                .unwrap()
        });
        for n in nbrs.iter() {
            if table.get_obstacle(n.0, n.1) == Obstacle::Platform
                && !table.blocked.contains_key(&(n.0, n.1))
            {
                slide.position = *n;
                break;
            }
        }
    }

    slide
}
