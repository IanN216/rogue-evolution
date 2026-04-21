pub const PARASANGA_SIZE: i32 = 64;
pub const WORLD_WIDTH_REGIONS: i32 = 8;
pub const WORLD_HEIGHT_REGIONS: i32 = 8;
pub const PLANET_TILE_WIDTH: i32 = PARASANGA_SIZE * WORLD_WIDTH_REGIONS;
pub const PLANET_TILE_HEIGHT: i32 = PARASANGA_SIZE * WORLD_HEIGHT_REGIONS;

// Parámetros de Ruido Avanzado (Spec: FBM + Domain Warping)
pub const GLOBAL_NOISE_SCALE: f64 = 0.5; 
pub const FBM_OCTAVES: usize = 6;
pub const FBM_PERSISTENCE: f64 = 0.5;
pub const FBM_LACUNARITY: f64 = 2.0;
