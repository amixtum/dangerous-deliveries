use model::{obstacle_table::ObstacleTable, obstacle::Obstacle, map_gen};
use rltk::{RandomNumberGenerator};


pub fn tunnel_spawn(obs_table: &mut ObstacleTable) -> (i32, i32) {
    let mut rng = RandomNumberGenerator::new();
    let x = (obs_table.width() as i32 / 2)
        + rng.range(
            -(obs_table.width() as i32) / 2 + 1,
            obs_table.width() as i32 / 2,
        )
        - 1;
    let y = (obs_table.height() as i32 / 2)
        + rng.range(
            -(obs_table.height() as i32) / 2 + 1,
            obs_table.height() as i32 / 2 - 1,
        );

    if !obs_table.blocked.contains_key(&(x, y)) {
        obs_table.set_obstacle((x, y), Obstacle::Platform);
        map_gen::tunnel_position(obs_table, (x, y));
    }

    (x, y)
}

pub fn random_platform(obs_table: &ObstacleTable) -> (i32, i32) {
    let mut rng = RandomNumberGenerator::new();
    let mut x = (obs_table.width() as i32 / 2)
        + rng.range(
            -(obs_table.width() as i32) / 2 + 1,
            obs_table.width() as i32 / 2,
        )
        - 1;
    let mut y = (obs_table.height() as i32 / 2)
        + rng.range(
            -(obs_table.height() as i32) / 2 + 1,
            obs_table.height() as i32 / 2 - 1,
        );

    while obs_table.blocked.contains_key(&(x, y)) || obs_table.get_obstacle(x, y) == Obstacle::Wall {
        x = (obs_table.width() as i32 / 2)
            + rng.range(
                -(obs_table.width() as i32) / 2 + 1,
                obs_table.width() as i32 / 2,
            )
            - 1;
        y = (obs_table.height() as i32 / 2)
            + rng.range(
                -(obs_table.height() as i32) / 2 + 1,
                obs_table.height() as i32 / 2 - 1,
            );       
    }

    (x, y)
}