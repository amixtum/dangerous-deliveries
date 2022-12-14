use controller::ai_controller::AIController;
use controller::player_controller::PlayerController;
use model::visibility;
use rltk::{Bresenham, FontCharType, Point, RGB};

use std::collections::HashMap;

use util::vec_ops;

use model::goal_table::GoalTable;
use model::obstacle::Obstacle;
use model::obstacle_table::ObstacleTable;
use model::player::Player;
use model::player_event::PlayerEvent;
use model::traversability::Traversability;

pub struct MainViewer {
    symbol_map: HashMap<Obstacle, FontCharType>,
    message_log: Vec<(String, RGB)>,
    log_length: usize,
    _max_message_length: u32,
}

impl MainViewer {
    pub fn new(log_length: usize) -> Self {
        let mut gv = MainViewer {
            symbol_map: HashMap::new(),
            message_log: Vec::new(),
            log_length,
            _max_message_length: 16,
        };

        gv.symbol_map.insert(Obstacle::Pit, rltk::to_cp437('x'));
        gv.symbol_map
            .insert(Obstacle::Platform, rltk::to_cp437('.'));
        gv.symbol_map.insert(Obstacle::Wall, rltk::to_cp437('#'));

        // bug (havent' found it yet)
        gv.symbol_map
            .insert(Obstacle::Rail(0, 0), rltk::to_cp437('_'));

        // right
        gv.symbol_map
            .insert(Obstacle::Rail(1, 0), rltk::to_cp437('>'));

        // left
        gv.symbol_map
            .insert(Obstacle::Rail(-1, 0), rltk::to_cp437('<'));

        // up
        gv.symbol_map
            .insert(Obstacle::Rail(0, -1), rltk::to_cp437('^'));

        // down
        gv.symbol_map
            .insert(Obstacle::Rail(0, 1), rltk::to_cp437('v'));

        // diagonal right up
        gv.symbol_map
            .insert(Obstacle::Rail(1, -1), rltk::to_cp437('/'));

        // diagonal left down
        gv.symbol_map
            .insert(Obstacle::Rail(-1, 1), rltk::to_cp437('d'));

        // diagonal right down
        gv.symbol_map
            .insert(Obstacle::Rail(1, 1), rltk::to_cp437('\\'));

        // diagonal left up
        gv.symbol_map
            .insert(Obstacle::Rail(-1, -1), rltk::to_cp437('u'));

        gv
    }

    pub fn direction_string((xdir, ydir): (i32, i32)) -> String {
        let mut s = String::new();
        if ydir == 1 {
            s.push_str("Down");
        } else if ydir == -1 {
            s.push_str("Up");
        }
        if xdir == 1 {
            s.push_str("Right");
        } else if xdir == -1 {
            s.push_str("Left");
        }

        s
    }
}

impl MainViewer {
    pub fn draw_layout(
        &self,
        ctx: &mut rltk::Rltk,
        table: &mut ObstacleTable,
        goals: &GoalTable,
        player: &Player,
        ai: &Vec<AIController>,
        controller: &PlayerController,
        max_speed: f32,
        fallover_threshold: f32,
        width: u32,
        height: u32,
        score: i32,
    ) {
        let speed_width = 8;
        let speed_tlx = width - speed_width - 1;
        let msg_log_height = speed_width as i32;
        let table_view_width = width;

        let table_view_height = height as i32 - msg_log_height - 3;
        let msg_log_tl_y = height as i32 - msg_log_height - 1;

        self.draw_table(
            ctx,
            0,
            1,
            table,
            goals,
            player,
            ai,
            controller,
            table_view_width as u32,
            table_view_height as u32,
            fallover_threshold,
        );
        self.draw_msg_log(
            ctx,
            0,
            msg_log_tl_y as i32,
            width - 1,
            msg_log_height as u32,
        );

        self.draw_speed(
            ctx,
            speed_tlx as i32,
            msg_log_tl_y,
            player,
            max_speed,
            speed_width as u32,
        );

        let mut s = String::new();
        s.push_str(&format!("${}, ", score));
        s.push_str("Help: press Esc");

        ctx.print_color(
            0,
            0,
            RGB::named(rltk::ALICEBLUE),
            RGB::named(rltk::BLACK),
            &s,
        );
    }

    // return a Screen of dimensions width x height that maps a width x height section
    // of the ObstacleTable centered on the player (any ObstacleTable coordinates that are out of bounds
    // are clamped out and the screen doesn't draw anything there)
    pub fn draw_table(
        &self,
        ctx: &mut rltk::Rltk,
        sc_tlx: i32,
        sc_tly: i32,
        table: &mut ObstacleTable,
        goals: &GoalTable,
        player: &Player,
        ai: &Vec<AIController>,
        controller: &PlayerController,
        width: u32,
        height: u32,
        fallover_threshold: f32,
    ) {
        // set to true to reveal the map and all entities
        let testing = true;

        // compute ObstacleTable coordinates
        let middle = player.xy();
        let mut tl_x = (middle.0 - (width / 2) as i32).clamp(0, table.width() as i32 - 1);
        let mut tl_y = (middle.1 - (height / 2) as i32).clamp(0, table.height() as i32 - 1);

        let mut br_x = (middle.0 + (width as i32 / 2)).clamp(0, table.width() as i32 - 1);
        let mut br_y = (middle.1 + (height as i32 / 2)).clamp(0, table.height() as i32 - 1);

        if br_x == table.width() as i32 - 1 {
            tl_x -= (middle.0 + width as i32 / 2) - br_x;
            tl_x = tl_x.clamp(0, table.width() as i32 - 1);
        } else if tl_x == 0 {
            br_x += (middle.0 + width as i32 / 2) - tl_x;
            br_x = br_x.clamp(0, table.width() as i32 - 1);
        }

        if br_y == table.height() as i32 - 1 {
            tl_y -= (middle.1 + height as i32 / 2) - br_y;
            tl_y = tl_y.clamp(0, table.height() as i32 - 1)
        } else if tl_y == 0 {
            br_y += (middle.1 + height as i32 / 2) - tl_y;
            br_y = br_y.clamp(0, table.height() as i32 - 1);
        }

        // screen coords
        let mut sc_x = sc_tlx;
        let mut sc_y = sc_tly;

        // put this somewhere else
        let visible = visibility::get_fov(player.xy(), table, 16);
        for p in visible.iter() {
            let idx = table.xy_flat(p.x, p.y) as usize;
            table.revealed[idx] = true;
            table.memory.push((p.x, p.y));
            if table.memory.len() > table.memory_size {
                let notvis = table.memory.remove(0);
                let idx = table.xy_flat(notvis.0, notvis.1) as usize;
                table.revealed[idx] = false;
            }
        }

        for x in tl_x..=br_x {
            for y in tl_y..=br_y {
                if testing || table.revealed[table.xy_flat(x, y)] || (player.x() == x && player.y() == y) || visible.contains(&Point::new(x, y)) {
                    let obstacle_type = table.get_obstacle(x, y);

                    let t = table.traversability((player.x(), player.y()), (x, y));
                    let symbol = self.symbol_map[&obstacle_type];

                    let mov = controller.move_player_vel(
                        table,
                        player,
                        (x as f32 - player.x() as f32, y as f32 - player.y() as f32),
                    );
                    let balance_amount = vec_ops::magnitude(mov.balance) / fallover_threshold;
                    let dist = vec_ops::magnitude((
                        x as f32 - player.x() as f32,
                        y as f32 - player.y() as f32,
                    ));
                    let inv_dist: f32;
                    if dist.round() as i32 == 0 {
                        inv_dist = 1.0;
                    } else {
                        inv_dist = 6.0 / (1.0 + dist);
                    }
                    match mov.recent_event {
                        PlayerEvent::FallOver | PlayerEvent::GameOver(_) => {
                            ctx.set(
                                sc_x,
                                sc_y,
                                RGB::from_f32(0.0, inv_dist, 0.0),
                                RGB::named(rltk::BLACK),
                                symbol,
                            );
                        }
                        _ => {
                            ctx.set(
                                sc_x,
                                sc_y,
                                RGB::from_f32(
                                    (1.0 - balance_amount) * inv_dist,
                                    0.0,
                                    balance_amount * inv_dist,
                                ),
                                RGB::named(rltk::BLACK),
                                symbol,
                            );
                        }
                    }

                    for goal in goals.goals.keys() {
                        if x == goal.0 && y == goal.1 && !goals.picked_up.contains(&(x, y)) {
                            if visible.contains(&Point::new(x, y)) {
                                match t {
                                    Traversability::No => {
                                        ctx.set(
                                            sc_x,
                                            sc_y,
                                            RGB::from_f32(inv_dist, inv_dist, inv_dist),
                                            RGB::named(rltk::BLACK),
                                            rltk::to_cp437('$'),
                                        );
                                    }
                                    _ => match mov.recent_event {
                                        PlayerEvent::FallOver | PlayerEvent::GameOver(_) => {
                                            ctx.set(
                                                sc_x,
                                                sc_y,
                                                RGB::from_f32(inv_dist, inv_dist, inv_dist),
                                                RGB::named(rltk::BLACK),
                                                rltk::to_cp437('$'),
                                            );
                                        }
                                        _ => {
                                            ctx.set(
                                                sc_x,
                                                sc_y,
                                                RGB::from_f32(1.0, 0.0, 0.0),
                                                RGB::named(rltk::WHITE),
                                                rltk::to_cp437('$'),
                                            );
                                        }
                                    },
                                }
                            }

                            break;
                        }
                    }

                    match obstacle_type {
                        Obstacle::Pit => {}
                        _ => {
                            for p in ai.iter().enumerate() {
                                let mut color = RGB::from_f32(inv_dist, inv_dist * 0.5, 0.0);
                                if let Some(key) = goals.index_map.get(&p.0) {
                                    if let Some((_idx, c)) = goals.goals.get(key) {
                                        color = *c;
                                    }
                                }
                                if x == p.1.player.x()
                                    && y == p.1.player.y()
                                    && table.blocked.contains_key(&(x, y))
                                    && (testing || visible.contains(&Point::new(x, y)))
                                {
                                    match p.1.player.recent_event {
                                        PlayerEvent::FallOver => {
                                            ctx.set(
                                                sc_x,
                                                sc_y,
                                                color,
                                                RGB::named(rltk::BLACK),
                                                rltk::to_cp437('!'),
                                            );
                                        }
                                        _ => {
                                            ctx.set(
                                                sc_x,
                                                sc_y,
                                                color,
                                                RGB::named(rltk::BLACK),
                                                rltk::to_cp437('???'),
                                            );
                                        }
                                    }
                                }
                            }

                            // draw player last so it is on top
                            if x == player.x() && y == player.y() {
                                match player.recent_event {
                                    PlayerEvent::FallOver => {
                                        ctx.set(
                                            sc_x,
                                            sc_y,
                                            RGB::named(rltk::WHITE),
                                            RGB::named(rltk::BLACK),
                                            rltk::to_cp437('!'),
                                        );
                                    }
                                    _ => {
                                        ctx.set(
                                            sc_x,
                                            sc_y,
                                            RGB::named(rltk::WHITE),
                                            RGB::named(rltk::BLACK),
                                            rltk::to_cp437('???'),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                sc_y += 1;
            }

            sc_y = sc_tly;
            sc_x += 1;
        }
    }

    // returns a Screen which visualizes the direction of the Player's
    // Balance vector, and their closeness to falling over (the nearness of the indicator to the border)
    pub fn draw_balance(
        &self,
        ctx: &mut rltk::Rltk,
        tlx: i32,
        tly: i32,
        player: &Player,
        fallover_threshold: f32,
        size: u32,
    ) {
        self.draw_vector(
            ctx,
            tlx,
            tly,
            player.balance,
            fallover_threshold,
            size,
            RGB::named(rltk::BLUE),
        );
    }

    pub fn draw_speed(
        &self,
        ctx: &mut rltk::Rltk,
        tlx: i32,
        tly: i32,
        player: &Player,
        max_speed: f32,
        size: u32,
    ) {
        self.draw_vector(
            ctx,
            tlx,
            tly,
            player.speed,
            max_speed,
            size,
            RGB::named(rltk::CYAN),
        );
    }

    pub fn draw_vector(
        &self,
        ctx: &mut rltk::Rltk,
        tlx: i32,
        tly: i32,
        v: (f32, f32),
        max: f32,
        size: u32,
        color: RGB,
    ) {
        // draw border
        ctx.draw_box(tlx, tly, size, size, color, RGB::named(rltk::BLACK));

        // compute position of vector inside the rect
        // is p_x correct?
        let p_x = (((v.0 / max) * (size as f32)).round() as i32 + (size as i32 / 2))
            .clamp(0, size as i32);
        let p_y = (((v.1 / max) * (size as f32)).round() as i32 + (size as i32 / 2))
            .clamp(0, size as i32);

        // indicate speed with this symbol
        let lines = Bresenham::new(
            Point::new(tlx + (size as i32 / 2), tly + (size as i32 / 2)),
            Point::new(tlx + p_x, tly + p_y),
        );
        ctx.set(
            tlx + (size as i32 / 2),
            tly + (size as i32 / 2),
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('o'),
        );
        for point in lines.skip(1) {
            ctx.set(
                point.x,
                point.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('.'),
            );
        }
        ctx.set(
            tlx + p_x,
            tly + p_y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('*'),
        );
    }

    pub fn draw_msg_log(&self, ctx: &mut rltk::Rltk, tlx: i32, tly: i32, width: u32, height: u32) {
        ctx.draw_box(
            tlx,
            tly,
            width,
            height,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );

        let mut l_index = (self.message_log.len() as i32 - 1) as i32;
        let mut scr_y = tly + 1;

        while scr_y < tly + height as i32 - 2 && l_index >= 0 {
            ctx.print_color(
                tlx + 1,
                scr_y,
                self.message_log[l_index as usize].1,
                RGB::named(rltk::BLACK),
                &self.message_log[l_index as usize].0,
            );

            scr_y += 1;
            l_index -= 1;
        }
    }

    pub fn add_string(&mut self, s: String, c: RGB) {
        self.message_log.push((s, c));
        if self.message_log.len() > self.log_length {
            self.message_log.remove(0);
        }
    }

    pub fn clear_log(&mut self) {
        self.message_log.clear();
    }
}
