use crate::components::kingdom::{KingdomRole, KingdomMember};
use hecs::World;

pub fn handle_kingdom_collapse(world: &mut World, fallen_kingdom_id: u32, conqueror_kingdom_id: u32) {
    let mut changes = Vec::new();

    // 1. Identify all citizens of the fallen kingdom
    for (entity, member) in world.query::<&KingdomMember>().iter() {
        if member.kingdom_id == fallen_kingdom_id {
            changes.push(entity);
        }
    }

    // 2. Assign new roles based on conqueror culture
    for entity in changes {
        let new_role = match conqueror_kingdom_id % 3 {
            0 => KingdomRole::Slave,
            1 => KingdomRole::Refugee,
            _ => KingdomRole::ExperimentSubject,
        };

        if let Ok(mut member) = world.get::<&mut KingdomMember>(entity) {
            member.kingdom_id = conqueror_kingdom_id;
            member.role = new_role;
        }
    }
}
