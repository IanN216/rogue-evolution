use hecs::World;
use bracket_lib::prelude::*;
use crate::components::stats::{Position, BaseStats, Renderable, Viewshed, Metabolism};
use crate::components::genetics::Genetics;
use crate::components::identity::Identity;
use crate::components::kingdom::KingdomMember;
use crate::components::progression::{Experience, AbilityRegistry, Humanoid};
use crate::components::items::{Item, Weapon, Blighted, InfectionSource};
use crate::core::world_map::{WorldMap, RegionData, EntitySnapshot, PARASANGA_SIZE};
use crate::utils::persistence::{save_region_async, load_region};

pub struct WorldManager {
    pub world: World,
    pub world_map: WorldMap,
}

impl WorldManager {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            world: World::new(),
            world_map: WorldMap::new(width, height),
        }
    }

    /// Vacía el mundo de entidades y reinicia el mapa (útil al volver al menú principal)
    pub fn clear(&mut self, width: i32, height: i32) {
        self.world = World::new();
        self.world_map = WorldMap::new(width, height);
    }

    /// Sistema de movimiento masivo optimizado para Celeron (Zero-Allocation)
    pub fn update_movement(&mut self) {
        // Reservado para lógica de movimiento masivo (Boids/Swarm)
        // Eliminado pos.x += 1 por ser destructivo para la simulación.
    }

    /// Implementa el Spec-5: Streaming de chunks (Parasangas) - Optimizado para Celeron
    pub fn stream_regions(&mut self, player_pos: Position) {
        let (px, py) = (player_pos.x / PARASANGA_SIZE, player_pos.y / PARASANGA_SIZE);
        
        self.world_map.loaded_regions.insert((px, py));

        let mut entities_to_remove = Vec::new();
        let mut snapshots_by_region: std::collections::HashMap<(i32, i32), Vec<EntitySnapshot>> = std::collections::HashMap::new();

        // Query optimizada para captura de snapshots
        for (entity, (pos, render, stats, view, gen, id, kingdom, metab, exp, abil, hum, itm, wpn, bli, inf)) in self.world.query_mut::<(
            &Position, 
            Option<&Renderable>, 
            Option<&BaseStats>,
            Option<&Viewshed>,
            Option<&Genetics>,
            Option<&Identity>,
            Option<&KingdomMember>,
            Option<&Metabolism>,
            Option<&Experience>,
            Option<&AbilityRegistry>,
            Option<&Humanoid>,
            Option<&Item>,
            Option<&Weapon>,
            Option<&Blighted>,
            Option<&InfectionSource>
        )>() {
            let (rx, ry) = (pos.x / PARASANGA_SIZE, pos.y / PARASANGA_SIZE);
            let dist = ((rx - px).pow(2) + (ry - py).pow(2)) as f32;
            
            if dist > 4.0 { 
                entities_to_remove.push(entity);
                
                snapshots_by_region.entry((rx, ry)).or_insert(Vec::new()).push(EntitySnapshot {
                    position: *pos,
                    renderable: render.cloned(),
                    base_stats: stats.cloned(),
                    viewshed: view.cloned(),
                    genetics: gen.cloned(),
                    identity: id.cloned(),
                    kingdom_member: kingdom.cloned(),
                    metabolism: metab.cloned(),
                    experience: exp.cloned(),
                    abilities: abil.cloned(),
                    is_humanoid: hum.is_some(),
                    item: itm.cloned(),
                    weapon: wpn.cloned(),
                    is_blighted: bli.is_some(),
                    is_infection_source: inf.is_some(),
                });
            }
        }

        for ((rx, ry), entities) in snapshots_by_region {
            // FIX: Guardar teselas actuales para evitar vacíos al recargar
            save_region_async(RegionData { 
                x: rx, 
                y: ry, 
                tiles: self.world_map.map.tiles.clone(), 
                entities 
            });
            self.world_map.loaded_regions.remove(&(rx, ry));
        }

        for entity in entities_to_remove {
            let _ = self.world.despawn(entity);
        }

        // Carga de regiones optimizada: Spawn agrupado para evitar archetype migration masivo
        for dx in -2..=2 {
            for dy in -2..=2 {
                let rx = px + dx;
                let ry = py + dy;
                if (dx*dx + dy*dy) <= 4 && !self.world_map.loaded_regions.contains(&(rx, ry)) {
                    if let Ok(region) = load_region(rx, ry) {
                        for snp in region.entities {
                            // Usamos una tupla de componentes comunes para que nazca en un arquetipo estable.
                            // Esto es mucho más rápido para el caché del Celeron.
                            let e = self.world.spawn((
                                snp.position,
                                snp.renderable.unwrap_or(Renderable { glyph: to_cp437('?'), fg: RGB::named(RED), bg: RGB::named(BLACK) }),
                                snp.base_stats.unwrap_or(BaseStats { hp: 1, max_hp: 1, attack: 0, defense: 0 }),
                                snp.identity.unwrap_or(Identity { name: "Unknown".to_string(), title: None, kingdom_id: 0 }),
                            ));

                            // Solo usamos insert_one para componentes variables o de menor frecuencia
                            if let Some(c) = snp.viewshed { self.world.insert_one(e, c).unwrap(); }
                            if let Some(c) = snp.genetics { self.world.insert_one(e, c).unwrap(); }
                            if let Some(c) = snp.kingdom_member { self.world.insert_one(e, c).unwrap(); }
                            if let Some(c) = snp.metabolism { self.world.insert_one(e, c).unwrap(); }
                            if let Some(c) = snp.experience { self.world.insert_one(e, c).unwrap(); }
                            if let Some(c) = snp.abilities { self.world.insert_one(e, c).unwrap(); }
                            if snp.is_humanoid { self.world.insert_one(e, Humanoid).unwrap(); }
                            if let Some(c) = snp.item { self.world.insert_one(e, c).unwrap(); }
                            if let Some(c) = snp.weapon { self.world.insert_one(e, c).unwrap(); }
                            if snp.is_blighted { self.world.insert_one(e, Blighted).unwrap(); }
                            if snp.is_infection_source { self.world.insert_one(e, InfectionSource).unwrap(); }
                        }
                        self.world_map.loaded_regions.insert((rx, ry));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::identity::Identity;

    #[test]
    fn test_persistence_streaming() {
        let mut manager = WorldManager::new(80, 50);
        let player_pos = Position { x: 0, y: 0 };
        
        // Spawn una entidad cerca del jugador
        manager.world.spawn((
            Position { x: 10, y: 10 },
            BaseStats { hp: 10, max_hp: 10, attack: 1, defense: 1 },
            Identity { name: "Nearby".to_string(), title: None, kingdom_id: 1 },
        ));

        // Spawn una entidad LEJOS del jugador (otra Parasanga)
        // PARASANGA_SIZE = 64. 2 Parasangas = 128.
        manager.world.spawn((
            Position { x: 200, y: 200 },
            BaseStats { hp: 50, max_hp: 50, attack: 5, defense: 5 },
            Identity { name: "Far Away".to_string(), title: None, kingdom_id: 2 },
        ));

        // Activar streaming
        manager.stream_regions(player_pos);

        // La entidad lejana debería haber sido eliminada del ECS
        let mut count = 0;
        for (_, id) in manager.world.query::<&Identity>().iter() {
            if id.name == "Far Away" { count += 1; }
        }
        assert_eq!(count, 0, "La entidad lejana no fue eliminada del ECS");

        // La entidad cercana debería seguir ahí
        let mut count_near = 0;
        for (_, id) in manager.world.query::<&Identity>().iter() {
            if id.name == "Nearby" { count_near += 1; }
        }
        assert_eq!(count_near, 1, "La entidad cercana fue eliminada incorrectamente");
        
        // Esperar un momento para que el hilo de guardado termine (o usar sync save para el test)
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Verificar que el archivo existe
        let rx = 200 / PARASANGA_SIZE;
        let ry = 200 / PARASANGA_SIZE;
        let filename = crate::utils::persistence::get_region_filename(rx, ry);
        assert!(std::path::Path::new("saves").join(filename).exists(), "El archivo de región no fue creado");
        
        // Limpiar
        let _ = std::fs::remove_dir_all("saves");
    }
}
