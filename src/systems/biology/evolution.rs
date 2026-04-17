use hecs::World;
use crate::components::genetics::{Genetics, PlagueMember};

pub fn process_evolution(world: &mut World, current_tick: u64) {
    // Run every 60 ticks
    if current_tick % 60 != 0 { return; }

    let mut transformations = Vec::new();

    // 1. Identify entities ready for evolution
    for (entity, gen) in world.query::<&Genetics>().iter() {
        if gen.exposure_level >= 100.0 {
            // Check if not already a PlagueMember
            if world.get::<&PlagueMember>(entity).is_err() {
                transformations.push(entity);
            }
        }
    }

    // 2. Apply archetype changes
    for entity in transformations {
        let _ = world.insert_one(entity, PlagueMember);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hecs::World;

    #[test]
    fn test_archetype_mutation() {
        let mut world = World::new();
        let entity = world.spawn((
            Genetics {
                dna: [0; 16],
                exposure_level: 110.0,
                generation: 1,
                race_id: 1,
                race_abilities: Vec::new(),
            },
        ));

        process_evolution(&mut world, 60);

        // Verify that PlagueMember component was added
        assert!(world.get::<&PlagueMember>(entity).is_ok());
    }
}
