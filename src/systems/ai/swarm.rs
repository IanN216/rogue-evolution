use hecs::World;
use crate::components::stats::Position;
use crate::components::kingdom::{HordeLeader, HordeMember};
use crate::components::ai::{DijkstraCache, HordeTarget};
use crate::core::map::Map;
use bracket_lib::prelude::*;
use rayon::prelude::*;

pub fn process_swarm_ai(world: &mut World, map: &Map) {
    // 1. Update Dijkstra Maps for leaders
    let mut maps_to_update = Vec::new();

    for (entity, (_pos, _leader, target)) in world.query::<(&Position, &HordeLeader, &HordeTarget)>().iter() {
        if let Ok(target_pos_comp) = world.get::<&Position>(target.target_entity) {
            let target_point = Point::new(target_pos_comp.x, target_pos_comp.y);
            
            let needs_update = if let Ok(cache) = world.get::<&DijkstraCache>(entity) {
                cache.target_pos != target_point
            } else {
                true
            };

            if needs_update {
                maps_to_update.push((entity, target_point));
            }
        }
    }

    for (entity, target_point) in maps_to_update {
        let start_indices = vec![map.xy_idx(target_point.x, target_point.y)];
        let mut dm = DijkstraMap::new(map.width, map.height, &start_indices, map, 1024.0);
        DijkstraMap::build(&mut dm, &start_indices, map);
        
        world.insert_one(entity, DijkstraCache { map: dm, target_pos: target_point }).unwrap();
    }

    // 2. Leader Movement
    let mut leader_movements = Vec::new();
    for (entity, (pos, _leader, cache)) in world.query::<(&Position, &HordeLeader, &DijkstraCache)>().iter() {
        let current_idx = map.xy_idx(pos.x, pos.y);
        if let Some(next_idx) = DijkstraMap::find_lowest_exit(&cache.map, current_idx, map) {
            leader_movements.push((entity, next_idx));
        }
    }

    for (entity, next_idx) in leader_movements {
        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            pos.x = (next_idx as i32) % map.width;
            pos.y = (next_idx as i32) / map.width;
        }
    }

    // 3. Member Movement (Parallel with Rayon)
    let mut leader_maps = std::collections::HashMap::new();
    let mut cache_query = world.query::<&DijkstraCache>().with::<&HordeLeader>();
    for (entity, cache) in cache_query.iter() {
        leader_maps.insert(entity, &cache.map);
    }

    let mut members_data = Vec::new();
    for (entity, (pos, member)) in world.query::<(&Position, &HordeMember)>().iter() {
        if let Some(dm) = leader_maps.get(&member.leader_entity) {
            members_data.push((entity, *pos, *dm));
        }
    }

    let movements: Vec<(hecs::Entity, i32, i32)> = members_data.par_iter().filter_map(|(entity, pos, dm)| {
        let current_idx = map.xy_idx(pos.x, pos.y);
        if let Some(next_idx) = DijkstraMap::find_lowest_exit(dm, current_idx, map) {
            Some((*entity, (next_idx as i32) % map.width, (next_idx as i32) / map.width))
        } else {
            None
        }
    }).collect();

    for (entity, next_x, next_y) in movements {
        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            pos.x = next_x;
            pos.y = next_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::TileType;
    use std::time::Instant;

    #[test]
    fn test_dijkstra_horde_performance() {
        let mut world = World::new();
        let mut map = Map::new(80, 50);
        for t in map.tiles.iter_mut() { *t = TileType::Floor; }
        
        // Add a wall in the middle
        for y in 10..40 { 
            let idx = map.xy_idx(40, y);
            map.tiles[idx] = TileType::Wall; 
        }

        let target = world.spawn((Position { x: 70, y: 25 },));
        let leader = world.spawn((
            Position { x: 10, y: 25 },
            HordeLeader,
            HordeTarget { target_entity: target }
        ));

        // Spawn 500 followers
        for i in 0..500 {
            world.spawn((
                Position { x: 5 + (i % 10), y: 20 + (i / 50) },
                HordeMember { leader_entity: leader }
            ));
        }

        let start = Instant::now();
        process_swarm_ai(&mut world, &map);
        let duration = start.elapsed();

        println!("Dijkstra Swarm AI (500 members) took: {:?}", duration);
        assert!(duration.as_millis() < 100, "Performance too slow: {:?}", duration);
    }

    #[test]
    fn test_horde_avoids_walls() {
        let mut world = World::new();
        let mut map = Map::new(80, 50);
        for t in map.tiles.iter_mut() { *t = TileType::Floor; }
        
        // Wall between (10,25) and (30,25)
        for x in 15..25 { 
            let idx = map.xy_idx(x, 25);
            map.tiles[idx] = TileType::Wall; 
        }

        let target = world.spawn((Position { x: 40, y: 25 },));
        let leader = world.spawn((
            Position { x: 10, y: 25 },
            HordeLeader,
            HordeTarget { target_entity: target }
        ));

        // Move leader
        process_swarm_ai(&mut world, &map);

        let pos = world.get::<&Position>(leader).unwrap();
        let idx = map.xy_idx(pos.x, pos.y);
        assert!(map.tiles[idx] == TileType::Floor, "Entidad terminó en un muro");
    }
}
