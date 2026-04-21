use super::map::{Map, TileType};
use crate::core::world_map::*;
use bracket_lib::prelude::*;
use rayon::prelude::*;
use noise::{NoiseFn, Simplex};
use std::f64::consts::PI;

pub fn build_planet(seed: u64) -> Map {
    let mut map = Map::new_planet();
    apply_advanced_geology(&mut map, seed);
    map
}

/// Función Helper: Fractal Brownian Motion en 4D para continuidad toroidal total.
/// Mapeamos el plano 2D a un toroide en 4D para eliminar costuras.
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

fn apply_advanced_geology(map: &mut Map, seed: u64) {
    let width = map.width as f64;
    let height = map.height as f64;
    
    // Generadores independientes para cada capa geográfica
    let n_elev = Simplex::new(seed as u32);
    let n_moist = Simplex::new((seed >> 16) as u32);
    let n_warp_q = Simplex::new((seed ^ 0x5555) as u32);
    let n_warp_r = Simplex::new((seed ^ 0xAAAA) as u32);

    map.tiles.par_iter_mut().enumerate().for_each(|(idx, tile)| {
        let x = (idx as i32 % map.width) as f64;
        let y = (idx as i32 / map.width) as f64;

        // 1. Proyección a Coordenadas Toroidales 4D
        // p.x -> (cos(x), sin(x)), p.y -> (cos(y), sin(y))
        let s = x / width;
        let t = y / height;
        
        let dx = (2.0 * PI * s).cos() * GLOBAL_NOISE_SCALE;
        let dy = (2.0 * PI * s).sin() * GLOBAL_NOISE_SCALE;
        let dz = (2.0 * PI * t).cos() * GLOBAL_NOISE_SCALE;
        let dw = (2.0 * PI * t).sin() * GLOBAL_NOISE_SCALE;

        // 2. Domain Warping (Noise Distortion) para formas orgánicas
        let q_x = fbm_4d(&n_warp_q, dx, dy, dz, dw, 3);
        let q_y = fbm_4d(&n_warp_q, dx + 5.2, dy + 1.3, dz + 0.5, dw + 2.1, 3);

        let r_x = fbm_4d(&n_warp_r, dx + 4.0 * q_x + 1.7, dy + 4.0 * q_y + 9.2, dz, dw, 3);
        let r_y = fbm_4d(&n_warp_r, dx + 4.0 * q_x + 8.3, dy + 4.0 * q_y + 2.8, dz, dw, 3);

        // Coordenadas finales distorsionadas
        let warp_x = dx + 4.0 * r_x;
        let warp_y = dy + 4.0 * r_y;

        // 3. Cálculo de Elevación y Humedad Final con FBM
        let elevation = fbm_4d(&n_elev, warp_x, warp_y, dz, dw, FBM_OCTAVES);
        let moisture = fbm_4d(&n_moist, dx, dy, dz, dw, FBM_OCTAVES);

        // 4. Matriz de Decisión Biomas (Climograph simplificado)
        *tile = if elevation < -0.35 {
            TileType::DeepWater
        } else if elevation < -0.15 {
            TileType::ShallowWater
        } else if elevation < -0.05 {
            TileType::Sand
        } else if elevation > 0.5 {
            if elevation > 0.7 { TileType::Snow } else { TileType::Mountain }
        } else {
            // Tierra firme: Depende de la humedad
            if moisture > 0.4 {
                TileType::Forest
            } else if moisture < -0.3 {
                TileType::StonyFloor // Representa desierto o estepa seca
            } else {
                TileType::Grass
            }
        };

        // Post-procesado: Semillas de vegetación o rocas
        if *tile == TileType::Grass || *tile == TileType::Forest {
            let mut rng = RandomNumberGenerator::seeded(seed + idx as u64);
            if rng.range(0, 100) < 5 { *tile = TileType::Wall; }
        }
    });
}
