use super::map::{Map, TileType};
use bracket_lib::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;

pub fn generate_caverns(width: i32, height: i32, seed: u64) -> Map {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut map = Map::new(width, height);

    // Initial random distribution
    for tile in map.tiles.iter_mut() {
        let roll = rng.gen_range(0..100);
        if roll < 45 { *tile = TileType::Floor; }
        else { *tile = TileType::Wall; }
    }

    // Apply Cellular Automata rules
    for _ in 0..10 {
        let mut new_tiles = map.tiles.clone();
        
        // Parallel processing using Rayon
        new_tiles.par_iter_mut().enumerate().for_each(|(idx, tile)| {
            let x = idx as i32 % width;
            let y = idx as i32 / width;
            
            // Skip boundaries
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                let neighbors = count_neighbors(&map, x, y);
                if neighbors > 4 || neighbors == 0 { *tile = TileType::Wall; }
                else { *tile = TileType::Floor; }
            } else {
                *tile = TileType::Wall;
            }
        });
        map.tiles = new_tiles;
    }

    ensure_connectivity(&mut map);
    map.populate_blocked();
    map
}

pub fn drunkard_walk(width: i32, height: i32, seed: u64) -> Map {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut map = Map::new(width, height);
    
    let mut drunk_x = width / 2;
    let mut drunk_y = height / 2;
    let mut floor_count = 0;
    let target_floor = (width * height) / 3; // 33% of the map

    while floor_count < target_floor {
        let idx = map.xy_idx(drunk_x, drunk_y);
        if map.tiles[idx] == TileType::Wall {
            map.tiles[idx] = TileType::Floor;
            floor_count += 1;
        }

        match rng.gen_range(0..4) {
            0 => if drunk_x > 1 { drunk_x -= 1; }
            1 => if drunk_x < width - 2 { drunk_x += 1; }
            2 => if drunk_y > 1 { drunk_y -= 1; }
            _ => if drunk_y < height - 2 { drunk_y += 1; }
        }
    }

    map.populate_blocked();
    map
}

fn count_neighbors(map: &Map, x: i32, y: i32) -> usize {
    let mut neighbors = 0;
    for iy in -1..=1 {
        for ix in -1..=1 {
            if ix == 0 && iy == 0 { continue; }
            let idx = map.xy_idx(x + ix, y + iy);
            if map.tiles[idx] == TileType::Wall { neighbors += 1; }
        }
    }
    neighbors
}

fn ensure_connectivity(map: &mut Map) {
    // Basic connectivity check: Find the largest floor area and fill others with walls
    let mut visited = vec![false; map.tiles.len()];
    let mut regions: Vec<Vec<usize>> = Vec::new();

    for (idx, tile) in map.tiles.iter().enumerate() {
        if *tile == TileType::Floor && !visited[idx] {
            let mut region = Vec::new();
            let mut q = std::collections::VecDeque::new();
            q.push_back(idx);
            visited[idx] = true;

            while let Some(current) = q.pop_front() {
                region.push(current);
                let x = current as i32 % map.width;
                let y = current as i32 / map.width;

                for (ix, iy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + ix;
                    let ny = y + iy;
                    if nx > 0 && nx < map.width - 1 && ny > 0 && ny < map.height - 1 {
                        let n_idx = map.xy_idx(nx, ny);
                        if map.tiles[n_idx] == TileType::Floor && !visited[n_idx] {
                            visited[n_idx] = true;
                            q.push_back(n_idx);
                        }
                    }
                }
            }
            regions.push(region);
        }
    }

    if regions.is_empty() { return; }

    // Keep only the largest region
    regions.sort_by(|a, b| b.len().cmp(&a.len()));
    
    // Convert everything else to walls
    for i in 1..regions.len() {
        for idx in &regions[i] {
            map.tiles[*idx] = TileType::Wall;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caverns_connectivity() {
        let map = generate_caverns(80, 50, 12345);
        let floor_count = map.tiles.iter().filter(|&&t| t == TileType::Floor).count();
        
        // Connectivity check: Flood fill from a floor tile
        let start_idx = map.tiles.iter().position(|&t| t == TileType::Floor).unwrap();
        let mut visited = vec![false; map.tiles.len()];
        let mut q = std::collections::VecDeque::new();
        q.push_back(start_idx);
        visited[start_idx] = true;
        let mut count = 0;

        while let Some(current) = q.pop_front() {
            count += 1;
            let x = current as i32 % map.width;
            let y = current as i32 / map.width;
            for (ix, iy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let n_idx = map.xy_idx(x + ix, y + iy);
                if map.tiles[n_idx] == TileType::Floor && !visited[n_idx] {
                    visited[n_idx] = true;
                    q.push_back(n_idx);
                }
            }
        }

        assert_eq!(count, floor_count, "No todos los suelos son accesibles");
    }

    #[test]
    fn test_caverns_wall_density() {
        let map = generate_caverns(80, 50, 12345);
        let wall_count = map.tiles.iter().filter(|&&t| t == TileType::Wall).count();
        let density = wall_count as f32 / map.tiles.len() as f32;
        assert!(density < 0.80, "Demasiados muros: {}%", density * 100.0);
    }
}
