use super::map::{Map, TileType};
use crate::core::world_map::*;
use crate::core::pcg::rivers::apply_hydrology;
use crate::core::pcg::cellular_automata::apply_cellular_automata;
use crate::core::pcg::drunkard_walk::apply_drunkard_walk;
use bracket_lib::prelude::*;
use noise::{NoiseFn, Simplex};
use std::f64::consts::PI;

pub fn build_planet(_seed: u64) -> Map {
    Map::new_planet()
}

/// Helper for Whittaker Biome Selection with Climate Refinement
fn get_whittaker_biome(elevation: f64, moisture: f64) -> TileType {
    if elevation < -0.35 {
        return TileType::DeepWater;
    }
    if elevation < -0.15 {
        return TileType::ShallowWater;
    }
    if elevation < -0.05 {
        return TileType::Sand;
    }

    // High Altitude (Alpine/Tundra)
    if elevation > 0.6 {
        if moisture > 0.3 { return TileType::Snow; }
        return TileType::Tundra; // Alpine Tundra
    }

    if elevation > 0.4 {
        if moisture > 0.5 { return TileType::Tundra; }
        if moisture > 0.1 { return TileType::StonyFloor; }
        return TileType::Desert; // Cold Desert
    }

    // Temperate / Tropical
    if moisture > 0.7 { return TileType::Jungle; }
    if moisture > 0.4 { return TileType::Forest; }
    if moisture > 0.1 { return TileType::Grass; }
    if moisture > -0.1 { return TileType::Savanna; }
    if moisture > -0.4 { return TileType::StonyFloor; } // Steppe
    
    TileType::Desert
}

fn ridged_fbm_4d(noise: &Simplex, x: f64, y: f64, z: f64, w: f64, octaves: usize) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..octaves {
        let v = noise.get([x * frequency, y * frequency, z * frequency, w * frequency]);
        // Ridged: (1.0 - abs(v))^2
        let ridge = 1.0 - v.abs();
        total += ridge * ridge * amplitude;

        max_amplitude += amplitude;
        amplitude *= FBM_PERSISTENCE;
        frequency *= FBM_LACUNARITY;
    }

    total / max_amplitude
}

pub fn generate_chunk(key: ChunkKey, seed: u64) -> Vec<TileType> {
    let mut tiles = vec![TileType::DeepWater; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    let mut elevation_data = vec![0.0; (CHUNK_SIZE * CHUNK_SIZE) as usize];
    
    let n_elev = Simplex::new(seed as u32);
    let n_moist = Simplex::new((seed >> 16) as u32);
    let n_warp_q = Simplex::new((seed ^ 0x5555) as u32);
    let n_warp_r = Simplex::new((seed ^ 0xAAAA) as u32);

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let world_x = (key.x * CHUNK_SIZE + x) as f64;
            let world_y = (key.y * CHUNK_SIZE + y) as f64;

            let s = world_x / PLANET_TILE_WIDTH as f64;
            let t = world_y / PLANET_TILE_HEIGHT as f64;

            let dx = (2.0 * PI * s).cos() * GLOBAL_NOISE_SCALE;
            let dy = (2.0 * PI * s).sin() * GLOBAL_NOISE_SCALE;
            let dz = (2.0 * PI * t).cos() * GLOBAL_NOISE_SCALE;
            let dw = (2.0 * PI * t).sin() * GLOBAL_NOISE_SCALE;

            let q_x = fbm_4d(&n_warp_q, dx, dy, dz, dw, 6);
            let q_y = fbm_4d(&n_warp_q, dx + 5.2, dy + 1.3, dz + 0.5, dw + 2.1, 6);

            let r_x = fbm_4d(&n_warp_r, dx + 4.0 * q_x + 1.7, dy + 4.0 * q_y + 9.2, dz, dw, 6);
            let r_y = fbm_4d(&n_warp_r, dx + 4.0 * q_x + 8.3, dy + 4.0 * q_y + 2.8, dz, dw, 6);

            let warp_x = dx + 6.0 * r_x;
            let warp_y = dy + 6.0 * r_y;

            let base_elev = fbm_4d(&n_elev, warp_x, warp_y, dz, dw, FBM_OCTAVES);
            let elevation = if base_elev > 0.2 {
                let ridge_v = ridged_fbm_4d(&n_elev, warp_x * 1.5, warp_y * 1.5, dz, dw, FBM_OCTAVES);
                let t = ((base_elev - 0.2) / 0.4).clamp(0.0, 1.0);
                base_elev * (1.0 - t) + ridge_v * t
            } else {
                base_elev
            };

            let mut moisture = fbm_4d(&n_moist, dx, dy, dz, dw, FBM_OCTAVES);

            // Shadow Rain Effect
            let east_x = (world_x + 32.0).rem_euclid(PLANET_TILE_WIDTH as f64) / PLANET_TILE_WIDTH as f64;
            let ex = (2.0 * PI * east_x).cos() * GLOBAL_NOISE_SCALE;
            let ey = (2.0 * PI * east_x).sin() * GLOBAL_NOISE_SCALE;
            let elev_east = fbm_4d(&n_elev, ex, ey, dz, dw, 3);
            
            if elev_east > 0.4 && elevation < elev_east {
                moisture -= (elev_east - 0.4) * 0.8;
            }

            let idx = (y * CHUNK_SIZE + x) as usize;
            elevation_data[idx] = elevation;
            tiles[idx] = get_whittaker_biome(elevation, moisture);
        }
    }

    // Apply Hydrology
    apply_hydrology(&mut tiles, &elevation_data);

    // Apply PCG algorithms for local detail refinement
    if tiles.iter().any(|&t| t == TileType::Forest || t == TileType::Jungle) {
        apply_cellular_automata(&mut tiles, key, seed, TileType::Grass, TileType::Forest);
    }

    // Example: Use Drunkard's Walk for mountain paths or caves
    if tiles.iter().any(|&t| t == TileType::Mountain) {
        let mut rng = RandomNumberGenerator::seeded(seed + key.x as u64 + key.y as u64);
        let start_x = rng.range(0, CHUNK_SIZE);
        let start_y = rng.range(0, CHUNK_SIZE);
        apply_drunkard_walk(&mut tiles, TileType::StonyFloor, start_x, start_y, 50);
    }

    tiles
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
