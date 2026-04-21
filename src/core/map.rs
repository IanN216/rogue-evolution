use crate::core::world_map::{PLANET_TILE_WIDTH, PLANET_TILE_HEIGHT, ChunkKey, CHUNK_SIZE};
use bracket_lib::prelude::*;
use std::collections::HashMap;
use hecs::World;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
    Snow,
    Wall,
    StonyFloor,
    MuddyFloor,
    Tundra,
    Jungle,
    Savanna,
    Desert,
}

pub struct Map {
    pub chunks: HashMap<ChunkKey, Vec<TileType>>,
    pub width: i32,
    pub height: i32,
    pub world: World, // hecs ECS World
}

impl Map {
    pub fn new_planet() -> Map {
        Map {
            chunks: HashMap::new(),
            width: PLANET_TILE_WIDTH,
            height: PLANET_TILE_HEIGHT,
            world: World::new(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        let world_x = x.rem_euclid(self.width);
        let world_y = y.rem_euclid(self.height);
        let key = ChunkKey::from_world_coords(world_x, world_y);
        
        if let Some(chunk) = self.chunks.get(&key) {
            let local_x = world_x.rem_euclid(CHUNK_SIZE);
            let local_y = world_y.rem_euclid(CHUNK_SIZE);
            let idx = (local_y * CHUNK_SIZE + local_x) as usize;
            chunk[idx]
        } else {
            TileType::DeepWater // Fallback for ungenerated chunks
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, _idx: usize) -> bool {
        // This is tricky with chunks. BaseMap might need a different approach 
        // if bracket-lib algorithms expect a flat array.
        // For now, let's keep it simple or implement it if needed.
        false
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
