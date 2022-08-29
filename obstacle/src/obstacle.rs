use point_vertex::point_vertex::PointVertex;

pub enum Obstacle {
    Platform(PointVertex),
    Pit(PointVertex),
    Rail(PointVertex),
}
