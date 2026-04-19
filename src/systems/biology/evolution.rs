use hecs::World;
use crate::components::genetics::{Genetics, PlagueMember};

pub fn process_evolution(world: &mut World, _current_tick: u64) {
    let mut to_evolve = Vec::new();

    for (entity, genetics) in world.query::<&Genetics>().iter() {
        // Check if specific traits are expressed for evolution
        let mut plague_score = 0;
        for gene in genetics.dna.iter() {
            if gene.trait_id == "plague" && gene.is_expressed() {
                plague_score += 1;
            }
        }

        if plague_score > 0 {
            // Check if already evolved
            if world.get::<&PlagueMember>(entity).is_err() {
                to_evolve.push(entity);
            }
        }
    }

    for entity in to_evolve {
        let _ = world.insert_one(entity, PlagueMember);
        // Additional changes like stats boosts could go here
    }
}
