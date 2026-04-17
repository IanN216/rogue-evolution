use bracket_lib::prelude::*;
use crate::core::map::Map;

pub fn compute_fov(origin: Point, range: i32, map: &Map) -> Vec<Point> {
    field_of_view(origin, range, map)
}
