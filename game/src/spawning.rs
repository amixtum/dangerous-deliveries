use model::{map_gen, obstacle::Obstacle, obstacle_table::ObstacleTable};
use rltk::RandomNumberGenerator;

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

    obs_table.set_obstacle((x, y), Obstacle::Platform);
    map_gen::tunnel_position(obs_table, (x, y));

    (x, y)
}

pub fn random_platform(obs_table: &ObstacleTable) -> (i32, i32) {
    let mut tries = 0;
    let mut rng = RandomNumberGenerator::new();
    if let Some(pos) = rng.random_slice_entry(&obs_table.platforms) {
        let mut pos = *pos;
        while obs_table.blocked.contains_key(&pos) && tries < obs_table.width() {
            let try_pos = rng.random_slice_entry(&obs_table.platforms);
            match try_pos {
                None => return (-1, -1),
                Some(next_pos) => {
                    if next_pos.0 != pos.0 || next_pos.1 != pos.1 {
                        pos = *next_pos;
                    }
                }
            }
            tries += 1;
        }
        return (pos.0, pos.1);
    }
    (-1, -1)
}
