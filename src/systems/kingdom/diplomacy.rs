use crate::components::kingdom::{KingdomState, HordeLeader, HordeMember};
use hecs::World;

pub struct DiplomacySystem {
    pub current_day: u64,
}

impl DiplomacySystem {
    pub fn new() -> Self {
        Self { current_day: 0 }
    }

    pub fn process_tick(&mut self, world: &mut World) {
        self.current_day += 1;
        if self.current_day % 30 == 0 {
            self.evaluate_geopolitics(world);
        }
    }

    fn evaluate_geopolitics(&self, world: &mut World) {
        for (_entity, state) in world.query::<&mut KingdomState>().iter() {
            if !state.is_active { continue; }
            if state.corruption > 80.0 && state.resources < 20.0 {
                // Potential invasion trigger
            }
        }
    }
}

pub fn ensure_horde_continuity(world: &mut World) {
    let mut orphan_hordes = Vec::new();

    for (entity, member) in world.query::<&HordeMember>().iter() {
        if world.get::<&HordeLeader>(member.leader_entity).is_err() {
            orphan_hordes.push((entity, member.leader_entity));
        }
    }

    for (follower, _) in orphan_hordes {
        let _ = world.insert_one(follower, HordeLeader);
        let _ = world.remove_one::<HordeMember>(follower);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::stats::Position;
    use hecs::World;

    #[test]
    fn test_diplomacy_tick_30_days() {
        let mut world = World::new();
        let mut system = DiplomacySystem::new();
        
        world.spawn((
            KingdomState {
                name: "Test Kingdom".to_string(),
                resources: 10.0,
                corruption: 90.0,
                order: 50.0,
                is_active: true,
            },
        ));

        for _ in 0..30 {
            system.process_tick(&mut world);
        }

        assert_eq!(system.current_day, 30);
    }

    #[test]
    fn test_horde_leader_backup() {
        let mut world = World::new();
        
        let leader = world.spawn((Position { x: 10, y: 10 }, HordeLeader));
        let follower = world.spawn((Position { x: 11, y: 11 }, HordeMember { leader_entity: leader }));

        world.despawn(leader).unwrap();
        ensure_horde_continuity(&mut world);

        assert!(world.get::<&HordeLeader>(follower).is_ok());
    }
}
