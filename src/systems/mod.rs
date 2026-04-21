pub mod ai;
pub mod biology;
pub mod kingdom;
pub mod sensory;
pub mod progression;
pub mod items;

use hecs::World;
use crate::components::stats::Position;
use crate::core::world_map::{PARASANGA_SIZE, WORLD_WIDTH_REGIONS, WORLD_HEIGHT_REGIONS};

/// Sistema que unifica las coordenadas del ECS dentro de la topología toroidal del planeta.
/// Se debe ejecutar al final de cada turno.
pub fn coordinate_unification(world: &mut World) {
    let planet_width = PARASANGA_SIZE * WORLD_WIDTH_REGIONS;
    let planet_height = PARASANGA_SIZE * WORLD_HEIGHT_REGIONS;

    for (_entity, pos) in world.query_mut::<&mut Position>() {
        pos.x = pos.x.rem_euclid(planet_width);
        pos.y = pos.y.rem_euclid(planet_height);
    }
}
