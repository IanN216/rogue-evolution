use hecs::World;
use bracket_lib::prelude::*;
use crate::components::stats::{Position, Viewshed, LightSource};
use crate::core::map::Map;
use crate::core::chronometry::TimeState;
use crate::utils::fov::compute_fov;

pub fn process_fov(world: &mut World, map: &Map, time: &TimeState) {
    let global_radius = time.get_visibility_radius();

    for (_entity, (pos, viewshed, light)) in world.query_mut::<(&Position, &mut Viewshed, Option<&LightSource>)>() {
        if viewshed.dirty {
            let base_range = viewshed.range;
            
            // El radio efectivo es afectado por el tiempo, pero LightSource da un mínimo.
            let mut effective_range = if base_range > global_radius {
                global_radius
            } else {
                base_range
            };

            if let Some(l) = light {
                if l.range > effective_range {
                    effective_range = l.range;
                }
            }

            viewshed.visible_tiles = compute_fov(Point::new(pos.x, pos.y), effective_range, map);
            viewshed.dirty = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::chronometry::TimeState;
    use crate::core::map::Map;

    #[test]
    fn test_fov_day_night_cycle() {
        let mut world = World::new();
        let mut map = Map::new(80, 50);
        for t in map.tiles.iter_mut() { *t = crate::core::map::TileType::Floor; }
        
        let mut time = TimeState::new();
        
        // Entity con viewshed de 30
        let entity = world.spawn((
            Position { x: 40, y: 25 },
            Viewshed { visible_tiles: Vec::new(), range: 30, dirty: true },
        ));

        // 1. Día (ticks = 800 -> 8:00 AM)
        time.ticks = 800;
        process_fov(&mut world, &map, &time);
        {
            let viewshed = world.get::<&Viewshed>(entity).unwrap();
            // En un mapa vacío de 80x50, un radio de 30 debería ver muchas celdas.
            // El radio de día es 30, así que debería usar 30.
            assert!(viewshed.visible_tiles.len() > 100);
        }

        // 2. Noche (ticks = 0 -> 12:00 AM)
        time.ticks = 0;
        {
            let mut viewshed = world.get::<&mut Viewshed>(entity).unwrap();
            viewshed.dirty = true;
        }
        process_fov(&mut world, &map, &time);
        {
            let viewshed = world.get::<&Viewshed>(entity).unwrap();
            // El radio de noche es 5, debería ver significativamente menos.
            // Un radio de 5 en shadowcasting ve ~80-100 celdas (pi * r^2 aprox)
            assert!(viewshed.visible_tiles.len() < 150);
        }
    }
}
