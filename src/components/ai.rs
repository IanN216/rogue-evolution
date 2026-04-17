use bracket_lib::prelude::*;

#[derive(Clone, Debug)]
pub struct HordeTarget {
    pub target_entity: hecs::Entity,
}

pub struct DijkstraCache {
    pub map: DijkstraMap,
    pub target_pos: Point,
}
