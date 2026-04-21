pub const PARASANGA_SIZE: i32 = 64;
pub const WORLD_WIDTH_REGIONS: i32 = 8;
pub const WORLD_HEIGHT_REGIONS: i32 = 8;
pub const PLANET_TILE_WIDTH: i32 = PARASANGA_SIZE * WORLD_WIDTH_REGIONS;
pub const PLANET_TILE_HEIGHT: i32 = PARASANGA_SIZE * WORLD_HEIGHT_REGIONS;

pub const CHUNK_SIZE: i32 = 32;
pub const VIEW_DISTANCE: i32 = 3; // Number of chunks to load around the player

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
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

// Parámetros de Ruido Avanzado (Spec: FBM + Domain Warping)
pub const GLOBAL_NOISE_SCALE: f64 = 0.5; 
pub const FBM_OCTAVES: usize = 6;
pub const FBM_PERSISTENCE: f64 = 0.5;
pub const FBM_LACUNARITY: f64 = 2.0;
