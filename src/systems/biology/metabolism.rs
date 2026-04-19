use hecs::World;
use crate::components::stats::{BaseStats, Metabolism};

pub fn process_metabolism(world: &mut World) {
    let query = world.query_mut::<(&mut BaseStats, &mut Metabolism)>();

    // Batching with Rayon or sequential processing for the 2-core Celeron
    // For now, simple sequential update
    for (_entity, (stats, metabolism)) in query {
        metabolism.hunger += metabolism.hunger_rate;

        if metabolism.hunger > metabolism.max_hunger {
            // Starvation: Lose 1 HP per tick
            stats.hp -= 1;
            metabolism.hunger = metabolism.max_hunger;
        }

        // Natural healing if not starving
        if stats.hp < stats.max_hp && metabolism.hunger < (metabolism.max_hunger * 0.5) {
            stats.hp += 1;
        }
    }
}
