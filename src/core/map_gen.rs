use super::map::{Map, TileType};
use bracket_lib::prelude::*;
use rayon::prelude::*;

pub fn generate_caverns(width: i32, height: i32, seed: u64) -> Map {
    let mut rng = RandomNumberGenerator::seeded(seed);
    let mut map = Map::new(width, height);

    // Initial random distribution
    for tile in map.tiles.iter_mut() {
        let roll = rng.range(0, 100);
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
                let neighbors = count_neighbors_static(&map.tiles, width, x, y);
                if neighbors > 4 || neighbors == 0 { *tile = TileType::Wall; }
                else { *tile = TileType::Floor; }
            } else {
                *tile = TileType::Wall;
            }
        });
        map.tiles = new_tiles;
    }

    let start_idx = map.xy_idx(width / 2, height / 2);
    ensure_connectivity(&mut map, start_idx);
    map.populate_blocked();
    map
}

// Fixed version of count_neighbors to avoid borrowing issues during parallel iter
fn count_neighbors_static(tiles: &[TileType], width: i32, x: i32, y: i32) -> usize {
    let mut neighbors = 0;
    for iy in -1..=1 {
        for ix in -1..=1 {
            if ix == 0 && iy == 0 { continue; }
            let idx = ((y + iy) * width + (x + ix)) as usize;
            if tiles[idx] == TileType::Wall { neighbors += 1; }
        }
    }
    neighbors
}

/// Implementación del Spec-1.1: Drunkard's Walk Regional con validación de conectividad
pub fn drunkard_walk(width: i32, height: i32, seed: u64) -> Map {
    let mut rng = RandomNumberGenerator::seeded(seed);
    let mut map = Map::new(width, height);
    
    // Iniciar con todo muros
    for tile in map.tiles.iter_mut() { *tile = TileType::Wall; }

    let mut floor_count = 0;
    let target_floor = (width * height) as f32 * 0.45; // 45% coverage

    while (floor_count as f32) < target_floor {
        let mut drunk_x = rng.range(1, width - 1);
        let mut drunk_y = rng.range(1, height - 1);
        let mut lifetime = 400; // Límite de pasos por caminante

        while lifetime > 0 {
            let idx = map.xy_idx(drunk_x, drunk_y);
            if map.tiles[idx] == TileType::Wall {
                map.tiles[idx] = TileType::Floor;
                floor_count += 1;
            }

            match rng.range(0, 4) {
                0 => if drunk_x > 1 { drunk_x -= 1; }
                1 => if drunk_x < width - 2 { drunk_x += 1; }
                2 => if drunk_y > 1 { drunk_y -= 1; }
                _ => if drunk_y < height - 2 { drunk_y += 1; }
            }

            lifetime -= 1;
            if (floor_count as f32) >= target_floor { break; }
        }
    }

    // Asegurar puntos de salida en los bordes para conectividad regional
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    // Tunel al Norte
    for y in 0..mid_y { let idx = map.xy_idx(mid_x, y); map.tiles[idx] = TileType::Floor; }
    // Tunel al Sur
    for y in mid_y..height { let idx = map.xy_idx(mid_x, y); map.tiles[idx] = TileType::Floor; }
    // Tunel al Oeste
    for x in 0..mid_x { let idx = map.xy_idx(x, mid_y); map.tiles[idx] = TileType::Floor; }
    // Tunel al Este
    for x in mid_x..width { let idx = map.xy_idx(x, mid_y); map.tiles[idx] = TileType::Floor; }

    // Validación post-generación (Objetivo #2 y #3)
    let start_idx = map.xy_idx(mid_x, mid_y);
    ensure_connectivity(&mut map, start_idx);

    map.populate_blocked();
    map
}

/// Verifica la conectividad desde un punto de referencia y elimina tiles huérfanos
fn ensure_connectivity(map: &mut Map, start_idx: usize) {
    let mut visited = vec![false; map.tiles.len()];
    let mut q = std::collections::VecDeque::new();

    // El punto de inicio debe ser Floor para el flood fill
    if map.tiles[start_idx] == TileType::Wall {
        map.tiles[start_idx] = TileType::Floor;
    }
    
    q.push_back(start_idx);
    visited[start_idx] = true;

    while let Some(current) = q.pop_front() {
        let x = current as i32 % map.width;
        let y = current as i32 / map.width;

        for (ix, iy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + ix;
            let ny = y + iy;
            if nx >= 0 && nx < map.width && ny >= 0 && ny < map.height {
                let n_idx = map.xy_idx(nx, ny);
                if map.tiles[n_idx] == TileType::Floor && !visited[n_idx] {
                    visited[n_idx] = true;
                    q.push_back(n_idx);
                }
            }
        }
    }

    // Manejo de "Tiles Huérfanos": Convertir tiles no alcanzables en muros
    for (idx, tile) in map.tiles.iter_mut().enumerate() {
        if *tile == TileType::Floor && !visited[idx] {
            *tile = TileType::Wall;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drunkard_deterministic_rng() {
        let seed = 123456789;
        let map1 = drunkard_walk(80, 50, seed);
        
        for i in 0..9 {
            let map2 = drunkard_walk(80, 50, seed);
            assert_eq!(map1.tiles, map2.tiles, "RNG no es determinista en la iteración {}", i);
        }
    }

    #[test]
    fn test_drunkard_coverage() {
        let map = drunkard_walk(80, 50, 42);
        let floor_count = map.tiles.iter().filter(|&&t| t == TileType::Floor).count();
        let coverage = floor_count as f32 / map.tiles.len() as f32;
        
        assert!(coverage >= 0.40 && coverage <= 0.55, "Cobertura fuera de rango: {}%", coverage * 100.0);
    }

    #[test]
    fn test_drunkard_connectivity_edges() {
        let map = drunkard_walk(80, 50, 99);
        // Verificar que los puntos cardinales medios sean Floor
        assert_eq!(map.tiles[map.xy_idx(40, 0)], TileType::Floor, "Falla salida Norte");
        assert_eq!(map.tiles[map.xy_idx(40, 49)], TileType::Floor, "Falla salida Sur");
        assert_eq!(map.tiles[map.xy_idx(0, 25)], TileType::Floor, "Falla salida Oeste");
        assert_eq!(map.tiles[map.xy_idx(79, 25)], TileType::Floor, "Falla salida Este");
    }
}
