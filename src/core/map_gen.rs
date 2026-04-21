use super::map::{Map, TileType};
use crate::core::world_map::*;
use crate::core::pcg::prefabs::{get_ruins_prefab, stamp_prefab};
use bracket_lib::prelude::*;
use noise::{NoiseFn, Simplex};
use std::f64::consts::PI;

pub fn build_planet(_seed: u64) -> Map {
    Map::new_planet()
}

fn get_whittaker_biome(elevation: f64, moisture: f64) -> TileType {
    if elevation < -0.45 { return TileType::DeepWater; }
    if elevation < -0.15 { return TileType::ShallowWater; }
    if elevation < -0.05 { return TileType::Sand; }
    if elevation > 0.65 {
        if moisture > 0.4 { return TileType::Snow; }
        return TileType::Tundra;
    }
    if elevation > 0.45 {
        if moisture > 0.6 { return TileType::Tundra; }
        if moisture > 0.2 { return TileType::StonyFloor; }
        return TileType::Desert;
    }
    if moisture > 0.75 { return TileType::Jungle; }
    if moisture > 0.45 { return TileType::Forest; }
    if moisture > 0.15 { return TileType::Grass; }
    if moisture > -0.1 { return TileType::Savanna; }
    TileType::Desert
}

fn ridged_fbm_4d(noise: &Simplex, x: f64, y: f64, z: f64, w: f64, octaves: usize) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;
    for _ in 0..octaves {
        let v = noise.get([x * frequency, y * frequency, z * frequency, w * frequency]);
        let ridge = 1.0 - v.abs();
        total += ridge * ridge * amplitude;
        max_amplitude += amplitude;
        amplitude *= FBM_PERSISTENCE;
        frequency *= FBM_LACUNARITY;
    }
    total / max_amplitude
}

fn fbm_4d(noise: &Simplex, x: f64, y: f64, z: f64, w: f64, octaves: usize) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;
    for _ in 0..octaves {
        total += noise.get([x * frequency, y * frequency, z * frequency, w * frequency]) * amplitude;
        max_amplitude += amplitude;
        amplitude *= FBM_PERSISTENCE;
        frequency *= FBM_LACUNARITY;
    }
    total / max_amplitude
}

pub fn generate_chunk(key: ChunkKey, seed: u64, global_rivers: &[River]) -> Vec<TileType> {
    let mut tiles = vec![TileType::DeepWater; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    let mut elevation_data = vec![0.0; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    let mut moisture_data = vec![0.0; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    
    let n_elev = Simplex::new(seed as u32);
    let n_moist = Simplex::new((seed >> 16) as u32);
    let n_warp = Simplex::new((seed ^ 0x5555) as u32);

    let pad_size = CHUNK_SIZE + 2;
    let mut pad_tiles = vec![TileType::DeepWater; (pad_size * pad_size) as usize];

    for y in -1..CHUNK_SIZE + 1 {
        for x in -1..CHUNK_SIZE + 1 {
            let world_x = (key.x * CHUNK_SIZE + x) as f64;
            let world_y = (key.y * CHUNK_SIZE + y) as f64;

            let s = world_x / PLANET_TILE_WIDTH as f64;
            let t = world_y / PLANET_TILE_HEIGHT as f64;
            let dx = (2.0 * PI * s).cos() * GLOBAL_NOISE_SCALE;
            let dy = (2.0 * PI * s).sin() * GLOBAL_NOISE_SCALE;
            let dz = (2.0 * PI * t).cos() * GLOBAL_NOISE_SCALE;
            let dw = (2.0 * PI * t).sin() * GLOBAL_NOISE_SCALE;

            let q_x = fbm_4d(&n_warp, dx, dy, dz, dw, 4);
            let q_y = fbm_4d(&n_warp, dx + 5.2, dy + 1.3, dz + 0.5, dw + 2.1, 4);
            let warp_x = dx + 3.0 * q_x; 
            let warp_y = dy + 3.0 * q_y;

            let base_elev = fbm_4d(&n_elev, warp_x, warp_y, dz, dw, FBM_OCTAVES);
            let elevation = if base_elev > 0.3 {
                let ridge_v = ridged_fbm_4d(&n_elev, warp_x * 2.0, warp_y * 2.0, dz, dw, FBM_OCTAVES);
                let t = ((base_elev - 0.3) / 0.3).clamp(0.0, 1.0);
                base_elev * (1.0 - t) + (base_elev + ridge_v * 0.5) * t
            } else {
                base_elev
            };

            let mut moisture = fbm_4d(&n_moist, dx, dy, dz, dw, FBM_OCTAVES);
            let east_x = (world_x + 20.0).rem_euclid(PLANET_TILE_WIDTH as f64) / PLANET_TILE_WIDTH as f64;
            let elev_east = fbm_4d(&n_elev, (2.0 * PI * east_x).cos() * GLOBAL_NOISE_SCALE, (2.0 * PI * east_x).sin() * GLOBAL_NOISE_SCALE, dz, dw, 3);
            if elev_east > 0.4 && elevation < elev_east {
                moisture -= (elev_east - 0.4) * 0.6;
            }

            let pad_idx = ((y + 1) * pad_size + (x + 1)) as usize;
            pad_tiles[pad_idx] = get_whittaker_biome(elevation, moisture);
            
            if x >= 0 && x < CHUNK_SIZE && y >= 0 && y < CHUNK_SIZE {
                let idx = (y * CHUNK_SIZE + x) as usize;
                elevation_data[idx] = elevation;
                moisture_data[idx] = moisture;
            }
        }
    }

    // Limpieza de Micro-Ruido y Transiciones Costeras
    let mut final_pad = pad_tiles.clone();
    for y in 1..pad_size-1 {
        for x in 1..pad_size-1 {
            let idx = (y * pad_size + x) as usize;
            let mut water_neighbors = 0;
            let mut land_neighbors = 0;
            
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let n_idx = ((y+dy)*pad_size + (x+dx)) as usize;
                    if pad_tiles[n_idx] == TileType::DeepWater || pad_tiles[n_idx] == TileType::ShallowWater {
                        water_neighbors += 1;
                    } else {
                        land_neighbors += 1;
                    }
                }
            }

            // Despeckling
            if water_neighbors == 8 && pad_tiles[idx] != TileType::DeepWater && pad_tiles[idx] != TileType::ShallowWater {
                final_pad[idx] = TileType::ShallowWater;
            } else if land_neighbors == 8 && (pad_tiles[idx] == TileType::DeepWater || pad_tiles[idx] == TileType::ShallowWater) {
                final_pad[idx] = TileType::Grass;
            }

            // Transiciones Costeras Estrictas
            if (pad_tiles[idx] == TileType::Forest || pad_tiles[idx] == TileType::Jungle || pad_tiles[idx] == TileType::Grass) {
                let mut touches_deep = false;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let n_idx = ((y+dy)*pad_size + (x+dx)) as usize;
                        if pad_tiles[n_idx] == TileType::DeepWater { touches_deep = true; break; }
                    }
                }
                if touches_deep {
                    final_pad[idx] = TileType::Sand;
                }
            }
        }
    }
    pad_tiles = final_pad;

    if pad_tiles.iter().any(|&t| t == TileType::Forest || t == TileType::Jungle) {
        apply_ca_buffered(&mut pad_tiles, pad_size, TileType::Grass, TileType::Forest);
    }

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let pad_idx = ((y + 1) * pad_size + (x + 1)) as usize;
            tiles[(y * CHUNK_SIZE + x) as usize] = pad_tiles[pad_idx];
        }
    }

    // Tallar rios globales con ensanchamiento
    for river in global_rivers {
        for &(rx, ry) in &river.path {
            let rx_rel = rx - key.x * CHUNK_SIZE;
            let ry_rel = ry - key.y * CHUNK_SIZE;
            
            if rx_rel >= -3 && rx_rel < CHUNK_SIZE + 3 && ry_rel >= -3 && ry_rel < CHUNK_SIZE + 3 {
                // Consultar elevacion global simplificada para el ancho
                let s = rx as f64 / PLANET_TILE_WIDTH as f64;
                let t = ry as f64 / PLANET_TILE_HEIGHT as f64;
                let elev = fbm_4d(&n_elev, (2.0 * PI * s).cos(), (2.0 * PI * s).sin(), (2.0 * PI * t).cos(), (2.0 * PI * t).sin(), 3);
                
                let radius = if elev > 0.4 { 0 } else if elev > 0.1 { 1 } else { 2 };
                
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let tx = rx_rel + dx;
                        let ty = ry_rel + dy;
                        if tx >= 0 && tx < CHUNK_SIZE && ty >= 0 && ty < CHUNK_SIZE {
                            let idx = (ty * CHUNK_SIZE + tx) as usize;
                            if tiles[idx] != TileType::DeepWater {
                                tiles[idx] = TileType::ShallowWater;
                            }
                        }
                    }
                }
            }
        }
    }

    // Estampar Prefabs POI
    let mut rng = RandomNumberGenerator::seeded(seed + key.x as u64 * 1000 + key.y as u64);
    if rng.range(0, 100) < 5 { // 5% de probabilidad por chunk
        let px = rng.range(5, CHUNK_SIZE - 10);
        let py = rng.range(5, CHUNK_SIZE - 10);
        let base_tile = tiles[(py * CHUNK_SIZE + px) as usize];
        if base_tile != TileType::DeepWater && base_tile != TileType::ShallowWater && base_tile != TileType::Mountain {
            let ruins = get_ruins_prefab();
            stamp_prefab(&mut tiles, &ruins, px, py);
        }
    }

    tiles
}

fn apply_ca_buffered(tiles: &mut [TileType], size: i32, floor: TileType, wall: TileType) {
    let mut new_tiles = tiles.to_vec();
    for _ in 0..2 {
        let prev = new_tiles.clone();
        for y in 1..size-1 {
            for x in 1..size-1 {
                let idx = (y * size + x) as usize;
                let mut n = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        if prev[((y+dy)*size + (x+dx)) as usize] == wall { n += 1; }
                    }
                }
                if n > 4 { new_tiles[idx] = wall; }
                else if n < 4 { new_tiles[idx] = floor; }
            }
        }
    }
    tiles.copy_from_slice(&new_tiles);
}
