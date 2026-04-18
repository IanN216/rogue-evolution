use bracket_lib::prelude::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HordeTarget {
    pub target_entity: hecs::Entity,
}

pub struct DijkstraCache {
    pub map: Arc<DijkstraMap>,
    pub target_pos: Point,
}
