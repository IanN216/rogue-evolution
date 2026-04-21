use crate::core::map::TileType;
use crate::core::world_map::*;
use noise::{NoiseFn, Simplex};

pub fn apply_cellular_automata(tiles: &mut [TileType], key: ChunkKey, seed: u64, floor_type: TileType, wall_type: TileType) {
    let mut new_tiles = tiles.to_vec();
    let noise = Simplex::new(seed as u32);

    for _ in 0..2 {
        let prev_tiles = new_tiles.clone();
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let idx = (y * CHUNK_SIZE + x) as usize;
                let mut neighbors = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        
                        let nx = x + dx;
                        let ny = y + dy;
                        
                        let is_wall = if nx >= 0 && nx < CHUNK_SIZE && ny >= 0 && ny < CHUNK_SIZE {
                            prev_tiles[(ny * CHUNK_SIZE + nx) as usize] == wall_type
                        } else {
                            // Muestreo determinista fuera del chunk usando ruido
                            let wx = (key.x * CHUNK_SIZE + nx) as f64;
                            let wy = (key.y * CHUNK_SIZE + ny) as f64;
                            noise.get([wx * 0.1, wy * 0.1]) > 0.0
                        };

                        if is_wall { neighbors += 1; }
                    }
                }

                if neighbors > 4 {
                    new_tiles[idx] = wall_type;
                } else if neighbors < 4 {
                    new_tiles[idx] = floor_type;
                }
            }
        }
    }
    tiles.copy_from_slice(&new_tiles);
}
