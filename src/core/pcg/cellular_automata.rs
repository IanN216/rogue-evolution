use crate::core::map::TileType;
use crate::core::world_map::CHUNK_SIZE;

pub fn apply_cellular_automata(tiles: &mut [TileType], floor_type: TileType, wall_type: TileType) {
    let mut new_tiles = tiles.to_vec();

    for _ in 0..4 {
        let prev_tiles = new_tiles.clone();
        for y in 1..CHUNK_SIZE-1 {
            for x in 1..CHUNK_SIZE-1 {
                let idx = (y * CHUNK_SIZE + x) as usize;
                let mut neighbors = 0;

                if prev_tiles[idx - 1] == wall_type { neighbors += 1; }
                if prev_tiles[idx + 1] == wall_type { neighbors += 1; }
                if prev_tiles[idx - CHUNK_SIZE as usize] == wall_type { neighbors += 1; }
                if prev_tiles[idx + CHUNK_SIZE as usize] == wall_type { neighbors += 1; }
                if prev_tiles[idx - CHUNK_SIZE as usize - 1] == wall_type { neighbors += 1; }
                if prev_tiles[idx - CHUNK_SIZE as usize + 1] == wall_type { neighbors += 1; }
                if prev_tiles[idx + CHUNK_SIZE as usize - 1] == wall_type { neighbors += 1; }
                if prev_tiles[idx + CHUNK_SIZE as usize + 1] == wall_type { neighbors += 1; }

                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = wall_type;
                } else {
                    new_tiles[idx] = floor_type;
                }
            }
        }
    }
    tiles.copy_from_slice(&new_tiles);
}
