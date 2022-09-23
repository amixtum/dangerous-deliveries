use std::collections::HashSet;

use rltk::Point;

use crate::obstacle_table::ObstacleTable;

pub fn get_fov(center: (i32, i32), table: &ObstacleTable, radius: i32) -> HashSet<Point> {
    rltk::field_of_view_set(Point::new(center.0, center.1), radius, table)
}
