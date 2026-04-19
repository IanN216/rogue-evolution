use hecs::World;
use crate::components::stats::Position;
use crate::components::items::{Blighted, InfectionSource};
use bracket_lib::prelude::*;

pub fn process_infection(world: &mut World, _current_tick: u64) {
    let mut to_infect = Vec::new();

    // Find all infection sources
    let mut sources = Vec::new();
    for (entity, (pos, _)) in world.query::<(&Position, &InfectionSource)>().iter() {
        sources.push((entity, pos.x, pos.y));
    }

    // Find potential targets near sources
    for (_entity, (pos, _)) in world.query::<(&Position, &Position)>().iter() { // Simplified query
        for (_src_entity, sx, sy) in sources.iter() {
            let dist = DistanceAlg::Pythagoras.distance2d(
                Point::new(pos.x, pos.y),
                Point::new(*sx, *sy)
            );

            if dist < 2.0 { // Proximity threshold
                to_infect.push(_entity);
            }
        }
    }

    // Apply infection (Blighted marker)
    for entity in to_infect {
        if world.get::<&Blighted>(entity).is_err() {
            let _ = world.insert_one(entity, Blighted);
        }
    }
}
