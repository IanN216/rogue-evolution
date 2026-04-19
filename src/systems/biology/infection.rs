use hecs::{World, Entity};
use crate::components::stats::Position;
use crate::components::items::{Blighted, InfectionSource};
use bracket_lib::prelude::*;

const CELL_SIZE: i32 = 4; // Potencia de 2 para optimizar con bit-shifting en Celeron

pub fn process_infection(world: &mut World, map_width: i32, map_height: i32) {
    let grid_w = (map_width / CELL_SIZE) + 1;
    let grid_h = (map_height / CELL_SIZE) + 1;
    let mut spatial_grid: Vec<Vec<Entity>> = vec![Vec::new(); (grid_w * grid_h) as usize];

    // 1. Llenado de la rejilla (Secuencial para evitar overhead de Mutex)
    for (entity, pos) in world.query::<&Position>().iter() {
        let cell_x = pos.x / CELL_SIZE;
        let cell_y = pos.y / CELL_SIZE;
        let idx = (cell_y * grid_w + cell_x) as usize;
        if idx < spatial_grid.len() {
            spatial_grid[idx].push(entity);
        }
    }

    let mut to_infect = Vec::new();

    // 2. Consulta de Proximidad (Cache-Friendly)
    // Solo iteramos sobre entidades que son fuentes de infección
    for (src_entity, (pos, _)) in world.query::<(&Position, &InfectionSource)>().iter() {
        let cell_x = pos.x / CELL_SIZE;
        let cell_y = pos.y / CELL_SIZE;

        // Comprobar celdas vecinas (incluida la propia)
        for dy in -1..=1 {
            for dx in -1..=1 {
                let cx = cell_x + dx;
                let cy = cell_y + dy;

                if cx >= 0 && cx < grid_w && cy >= 0 && cy < grid_h {
                    let idx = (cy * grid_w + cx) as usize;
                    for &target_entity in &spatial_grid[idx] {
                        if target_entity == src_entity { continue; }

                        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
                            let dist = DistanceAlg::Pythagoras.distance2d(
                                Point::new(pos.x, pos.y),
                                Point::new(target_pos.x, target_pos.y)
                            );

                            if dist < 2.0 {
                                to_infect.push(target_entity);
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Aplicar Infección (Blighted)
    for entity in to_infect {
        if world.get::<&Blighted>(entity).is_err() {
            let _ = world.insert_one(entity, Blighted);
        }
    }
}

