pub const PARASANGA_SIZE: i32 = 64;
pub const WORLD_WIDTH_REGIONS: i32 = 8;
pub const WORLD_HEIGHT_REGIONS: i32 = 8;
pub const PLANET_TILE_WIDTH: i32 = PARASANGA_SIZE * WORLD_WIDTH_REGIONS;
pub const PLANET_TILE_HEIGHT: i32 = PARASANGA_SIZE * WORLD_HEIGHT_REGIONS;

pub const CHUNK_SIZE: i32 = 32;
pub const VIEW_DISTANCE: i32 = 3; // Number of chunks to load around the player

use bracket_lib::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChunkKey {
    pub x: i32,
    pub y: i32,
}

impl ChunkKey {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Convert world coordinates to a ChunkKey
    pub fn from_world_coords(x: i32, y: i32) -> Self {
        Self {
            x: x.div_euclid(CHUNK_SIZE),
            y: y.div_euclid(CHUNK_SIZE),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct River {
    pub path: Vec<(i32, i32)>,
}

pub fn generate_global_rivers(seed: u64) -> Vec<River> {
    let mut rng = RandomNumberGenerator::seeded(seed);
    let mut rivers = Vec::new();

    // Generar 10 rios macro
    for _ in 0..10 {
        let mut path = Vec::new();
        let mut curr_x = rng.range(0, PLANET_TILE_WIDTH);
        let mut curr_y = rng.range(0, PLANET_TILE_HEIGHT);
        
        // Simulación muy simplificada de camino hacia el "mar" (borde o centro)
        for _ in 0..200 {
            path.push((curr_x, curr_y));
            // Moverse un poco aleatorio pero con tendencia
            curr_x = (curr_x + rng.range(-1, 3)).rem_euclid(PLANET_TILE_WIDTH);
            curr_y = (curr_y + rng.range(-1, 3)).rem_euclid(PLANET_TILE_HEIGHT);
        }
        rivers.push(River { path });
    }
    rivers
}

// Parámetros de Ruido Avanzado (Spec: FBM + Domain Warping)
pub const GLOBAL_NOISE_SCALE: f64 = 0.5; 
pub const FBM_OCTAVES: usize = 6;
pub const FBM_PERSISTENCE: f64 = 0.5;
pub const FBM_LACUNARITY: f64 = 2.0;
