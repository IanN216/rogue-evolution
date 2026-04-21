use crate::core::map::TileType;
use crate::core::world_map::CHUNK_SIZE;
use bracket_lib::prelude::*;

pub fn apply_drunkard_walk(tiles: &mut [TileType], floor_type: TileType, start_x: i32, start_y: i32, max_steps: usize) {
    let mut x = start_x;
    let mut y = start_y;
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..max_steps {
        let idx = (y * CHUNK_SIZE + x) as usize;
        tiles[idx] = floor_type;

        match rng.range(0, 4) {
            0 => x -= 1,
            1 => x += 1,
            2 => y -= 1,
            _ => y += 1,
        }

        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_SIZE {
            break;
        }
    }
}
