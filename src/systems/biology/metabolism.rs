use hecs::World;
use rayon::prelude::*;
use crate::components::stats::{BaseStats, Metabolism};

pub fn process_metabolism(world: &mut World) {
    let mut query = world.query_mut::<(&mut BaseStats, &mut Metabolism)>();
    
    // Batching with Rayon for the 2-core Celeron
    let mut targets: Vec<(&mut BaseStats, &mut Metabolism)> = query
        .into_iter()
        .map(|(_, (s, m))| (s, m))
        .collect();

    targets.par_chunks_mut(500).for_each(|chunk| {
        for (stats, metabolism) in chunk {
            metabolism.hunger += metabolism.hunger_rate;
            
            // Starvation impact
            if metabolism.hunger >= metabolism.max_hunger {
                stats.hp -= 1;
                metabolism.hunger = metabolism.max_hunger;
            }
        }
    });
}
