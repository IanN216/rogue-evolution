use crate::core::map::TileType;
use crate::core::world_map::CHUNK_SIZE;
use bracket_lib::prelude::*;

pub fn apply_hydrology(tiles: &mut [TileType], elevation_data: &[f64]) {
    // Encontrar puntos de inicio (picos de montaña)
    let mut starts = Vec::new();
    for (i, &e) in elevation_data.iter().enumerate() {
        if e > 0.6 {
            starts.push(i);
        }
    }

    if starts.is_empty() { return; }

    // Usar Dijkstra Map para encontrar el camino mas bajo (hacia el agua)
    // Nota: bracket-lib DijkstraMap suele usarse para encontrar caminos HACIA objetivos.
    // Aqui simularemos el flujo de agua moviendonos hacia la menor elevacion.
    
    for &start_idx in starts.iter().take(5) { // Limitar a algunos rios por chunk
        let mut curr_idx = start_idx;
        for _ in 0..100 { // Longitud maxima del rio
            let x = (curr_idx % CHUNK_SIZE as usize) as i32;
            let y = (curr_idx / CHUNK_SIZE as usize) as i32;

            if tiles[curr_idx] == TileType::DeepWater || tiles[curr_idx] == TileType::ShallowWater {
                break;
            }

            // Convertir a lodo o agua poco profunda
            if tiles[curr_idx] != TileType::Mountain && tiles[curr_idx] != TileType::Snow {
                tiles[curr_idx] = TileType::MuddyFloor;
            }

            // Encontrar el vecino con menor elevacion
            let mut best_neighbor = curr_idx;
            let mut min_elev = elevation_data[curr_idx];

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < CHUNK_SIZE && ny >= 0 && ny < CHUNK_SIZE {
                        let n_idx = (ny * CHUNK_SIZE + nx) as usize;
                        if elevation_data[n_idx] < min_elev {
                            min_elev = elevation_data[n_idx];
                            best_neighbor = n_idx;
                        }
                    }
                }
            }

            if best_neighbor == curr_idx { break; } // Pozo local
            curr_idx = best_neighbor;
        }
    }
}
