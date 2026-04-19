use hecs::{World, Entity};
use crate::components::stats::Position;
use crate::components::items::{Blighted, InfectionSource};
use bracket_lib::prelude::*;

const CELL_SIZE: i32 = 4; // 2^2
const CELL_SHIFT: i32 = 2; 

pub fn process_infection(world: &mut World, map_width: i32, map_height: i32) {
    let grid_w = (map_width >> CELL_SHIFT) + 1;
    let grid_h = (map_height >> CELL_SHIFT) + 1;
    let cell_count = (grid_w * grid_h) as usize;

    // Estructuras Planas (DOD) - Cache-Friendly
    let mut cell_counts = vec![0usize; cell_count];
    let mut entities_by_cell = Vec::with_capacity(world.len() as usize);
    let mut cell_starts = vec![0usize; cell_count];

    // 1. Fase de Conteo: O(n)
    for (_entity, pos) in world.query::<&Position>().iter() {
        let cell_x = pos.x >> CELL_SHIFT;
        let cell_y = pos.y >> CELL_SHIFT;
        let idx = (cell_y * grid_w + cell_x) as usize;
        if idx < cell_count {
            cell_counts[idx] += 1;
        }
    }

    // 2. Cálculo de Índices de Rango (Prefix Sum): O(grid)
    let mut current_start = 0;
    for i in 0..cell_count {
        cell_starts[i] = current_start;
        current_start += cell_counts[i];
    }

    // 3. Fase de Llenado del Búfer Plano: O(n)
    // Usamos un vector plano pre-dimensionado
    entities_by_cell.resize(current_start, Entity::DANGLING);
    let mut cell_offsets = vec![0usize; cell_count];
    for (entity, pos) in world.query::<&Position>().iter() {
        let cell_x = pos.x >> CELL_SHIFT;
        let cell_y = pos.y >> CELL_SHIFT;
        let idx = (cell_y * grid_w + cell_x) as usize;
        if idx < cell_count {
            let offset = cell_starts[idx] + cell_offsets[idx];
            entities_by_cell[offset] = entity;
            cell_offsets[idx] += 1;
        }
    }

    let mut to_infect = Vec::new();

    // 4. Consulta de Proximidad Masiva: O(fuentes * 9 celdas)
    for (src_entity, (pos, _)) in world.query::<(&Position, &InfectionSource)>().iter() {
        let cell_x = pos.x >> CELL_SHIFT;
        let cell_y = pos.y >> CELL_SHIFT;

        for dy in -1..=1 {
            for dx in -1..=1 {
                let cx = cell_x + dx;
                let cy = cell_y + dy;

                if cx >= 0 && cx < grid_w && cy >= 0 && cy < grid_h {
                    let idx = (cy * grid_w + cx) as usize;
                    let start = cell_starts[idx];
                    let end = start + cell_counts[idx];

                    for i in start..end {
                        let target_entity = entities_by_cell[i];
                        if target_entity == src_entity || target_entity == Entity::DANGLING { continue; }

                        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
                            let dist_sq = (pos.x - target_pos.x).pow(2) + (pos.y - target_pos.y).pow(2);
                            if dist_sq < 4 { // dist < 2.0 -> dist_sq < 4
                                to_infect.push(target_entity);
                            }
                        }
                    }
                }
            }
        }
    }

    // 5. Aplicación Diferida (Elimina Race Conditions)
    for entity in to_infect {
        if world.get::<&Blighted>(entity).is_err() {
            let _ = world.insert_one(entity, Blighted);
        }
    }
}
