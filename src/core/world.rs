use hecs::World;
use rayon::prelude::*;
use crate::components::stats::{Position, BaseStats};

pub struct WorldManager {
    pub world: World,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    /// Sistema de movimiento masivo optimizado para Celeron (Batching + Rayon)
    pub fn update_movement(&mut self) {
        let mut query = self.world.query_mut::<(&mut Position, &BaseStats)>();
        let mut targets: Vec<(&mut Position, &BaseStats)> = query.into_iter().map(|(_, (p, s))| (p, s)).collect();

        targets.par_chunks_mut(500).for_each(|chunk| {
            for (pos, _stats) in chunk {
                pos.x += 1;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::stats::{Position, BaseStats, Renderable};
    use crate::components::genetics::Genetics;
    use crate::components::identity::Identity;
    use bracket_lib::prelude::*;
    use std::time::Instant;

    #[test]
    fn test_ecs_query_performance() {
        let mut manager = WorldManager::new();
        
        // Spawn 10,000 entidades
        for i in 0..10_000 {
            manager.world.spawn((
                Position { x: i % 80, y: i / 80 },
                BaseStats { hp: 10, max_hp: 10, attack: 1, defense: 1 },
                Renderable { glyph: to_cp437('M'), fg: RGB::named(RED), bg: RGB::named(BLACK) },
                Genetics { 
                    dna: [0; 16], 
                    exposure_level: 0.0, 
                    generation: 1,
                    race_id: 1,
                    race_abilities: Vec::new(),
                },
                Identity { name: format!("Monster {}", i), title: None, kingdom_id: 1 },
            ));
        }

        let start = Instant::now();
        manager.update_movement();
        let duration = start.elapsed();
        
        println!("Tiempo de travesía para 10,000 entidades: {:?}", duration);
        assert!(duration.as_millis() < 100, "Rendimiento ECS insuficiente: {:?}", duration);
    }
}
