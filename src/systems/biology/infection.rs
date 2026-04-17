use hecs::{World, Entity};
use crate::components::stats::Position;
use crate::components::genetics::Genetics;
use std::collections::HashMap;

pub fn process_infection(world: &mut World, current_tick: u64) {
    // Skip frames to save CPU (run every 30 ticks)
    if current_tick % 30 != 0 { return; }

    let mut spatial_hash: HashMap<(i32, i32), Vec<Entity>> = HashMap::new();
    
    // 1. Populate spatial hash
    for (entity, (pos, _)) in world.query::<(&Position, &Genetics)>().iter() {
        let grid_pos = (pos.x / 2, pos.y / 2); // 2x2 grid cells
        spatial_hash.entry(grid_pos).or_insert_with(Vec::new).push(entity);
    }

    // 2. Check proximity and infect within cells
    // Using a simple logic: if an infected (high exposure) is near others, they share traits
    let mut mutations = Vec::new();

    for (_, entities) in spatial_hash.iter() {
        if entities.len() < 2 { continue; }

        for &e1 in entities {
            if let Ok(gen_a) = world.get::<&Genetics>(e1) {
                if gen_a.exposure_level > 50.0 {
                    // Entity e1 is a "carrier", potentially infects others in the same cell
                    for &e2 in entities {
                        if e1 == e2 { continue; }
                        mutations.push((e2, 1.0f32)); // Increase exposure of e2
                    }
                }
            }
        }
    }

    // 3. Apply results
    for (entity, increase) in mutations {
        if let Ok(mut genetics) = world.get::<&mut Genetics>(entity) {
            genetics.exposure_level += increase;
        }
    }
}
