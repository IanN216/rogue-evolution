use super::map::{Map, TileType};
use bracket_lib::prelude::*;
use rayon::prelude::*;
use noise::{NoiseFn, Simplex};

/// Fase 1: Composición de Ruido (Biomas) y Estructura Venosa Inicial
pub fn generate_caverns_step(map: &mut Map, iteration: usize, seed: u64) -> f32 {
    let width = map.width;
    let height = map.height;

    if iteration == 0 {
        // 1. Capa de Ruido Simplex para Biomas (Humedad y Temperatura)
        let simplex_moisture = Simplex::new(seed as u32);
        let simplex_temp = Simplex::new((seed >> 32) as u32);

        map.tiles.par_iter_mut().enumerate().for_each(|(idx, tile)| {
            let x = (idx as i32 % width) as f64 / 20.0;
            let y = (idx as i32 / width) as f64 / 20.0;
            
            let moisture = simplex_moisture.get([x, y]);
            let temp = simplex_temp.get([x, y]);

            // Determinar tipo de suelo base
            if moisture > 0.3 {
                *tile = TileType::MuddyFloor;
            } else if temp < -0.2 {
                *tile = TileType::StonyFloor;
            } else {
                *tile = TileType::Floor;
            }

            // Distribución inicial para CA (Venas)
            let mut rng = RandomNumberGenerator::seeded(seed + idx as u64);
            if rng.range(0, 100) < 48 {
                *tile = TileType::Wall;
            }
        });
        return 0.1;
    }

    // 2. Autómatas Celulares: Estructura Venosa (neighbors 4-6)
    let mut new_tiles = map.tiles.clone();
    new_tiles.par_iter_mut().enumerate().for_each(|(idx, tile)| {
        let x = idx as i32 % width;
        let y = idx as i32 / width;
        
        let neighbors = count_neighbors_static(&map.tiles, width, height, x, y);
        if neighbors >= 4 && neighbors <= 6 {
            *tile = TileType::Wall;
        } else {
            // Mantener el bioma original si no es muro
            if *tile == TileType::Wall {
                // Re-muestrear bioma basado en ruido global para costuras invisibles
                let simplex_moisture = Simplex::new(seed as u32);
                let simplex_temp = Simplex::new((seed >> 32) as u32);
                let moisture = simplex_moisture.get([x as f64 / 20.0, y as f64 / 20.0]);
                let temp = simplex_temp.get([x as f64 / 20.0, y as f64 / 20.0]);
                
                if moisture > 0.3 { *tile = TileType::MuddyFloor; }
                else if temp < -0.2 { *tile = TileType::StonyFloor; }
                else { *tile = TileType::Floor; }
            }
        }
    });
    map.tiles = new_tiles;
    
    (iteration as f32 / 10.0).min(1.0)
}

/// Fase 2: Drunkard's Walk con Ruido (Salidas Regionales No Lineales)
pub fn drunkard_walk_step(map: &mut Map, step: usize, seed: u64) -> f32 {
    let mut rng = RandomNumberGenerator::seeded(seed + step as u64);
    let width = map.width;
    let height = map.height;
    
    let target_floor = (width * height) as f32 * 0.45;
    let mut floor_count = map.tiles.iter().filter(|&&t| t != TileType::Wall).count();

    if (floor_count as f32) < target_floor {
        let mut drunk_x = rng.range(0, width);
        let mut drunk_y = rng.range(0, height);
        let mut lifetime = 300; 

        while lifetime > 0 {
            let idx = map.xy_idx(drunk_x, drunk_y);
            if map.tiles[idx] == TileType::Wall {
                map.tiles[idx] = TileType::Floor;
                floor_count += 1;
            }

            match rng.range(0, 4) {
                0 => drunk_x -= 1,
                1 => drunk_x += 1,
                2 => drunk_y -= 1,
                _ => drunk_y += 1,
            }
            
            drunk_x = drunk_x.rem_euclid(width);
            drunk_y = drunk_y.rem_euclid(height);

            lifetime -= 1;
            if (floor_count as f32) >= target_floor { break; }
        }
    }

    (floor_count as f32 / target_floor).min(1.0)
}

pub fn add_regional_exits(map: &mut Map) {
    let width = map.width;
    let height = map.height;
    let mut rng = RandomNumberGenerator::new();

    // Salidas con ruido (no líneas rectas)
    let mid_x = width / 2;
    let mid_y = height / 2;

    // Norte-Sur con Jitter (Toroidal)
    for y in 0..height {
        let jitter = rng.range(-2, 3);
        let x = mid_x + jitter;
        let idx = map.xy_idx(x, y);
        map.tiles[idx] = TileType::Floor;
    }

    // Este-Oeste con Jitter (Toroidal)
    for x in 0..width {
        let jitter = rng.range(-2, 3);
        let y = mid_y + jitter;
        let idx = map.xy_idx(x, y);
        map.tiles[idx] = TileType::Floor;
    }
}

/// Fase 3: Conectividad via A* y Puntos de Interés via Dijkstra
pub fn ensure_connectivity_step(map: &mut Map) {
    let mid_x = map.width / 2;
    let mid_y = map.height / 2;
    let start_idx = map.xy_idx(mid_x, mid_y);
    
    // 1. Identificar Islas (Flood Fill con Wrap)
    let mut visited = vec![false; map.tiles.len()];
    let mut groups = Vec::new();

    for i in 0..map.tiles.len() {
        if map.tiles[i] != TileType::Wall && !visited[i] {
            let mut group = Vec::new();
            let mut q = std::collections::VecDeque::new();
            q.push_back(i);
            visited[i] = true;

            while let Some(curr) = q.pop_front() {
                group.push(curr);
                let x = curr as i32 % map.width;
                let y = curr as i32 / map.width;
                for (ix, iy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + ix;
                    let ny = y + iy;
                    let n_idx = map.xy_idx(nx, ny);
                    if map.tiles[n_idx] != TileType::Wall && !visited[n_idx] {
                        visited[n_idx] = true;
                        q.push_back(n_idx);
                    }
                }
            }
            groups.push(group);
        }
    }

    // 2. Conectar Islas con A*
    if groups.len() > 1 {
        let main_group = &groups[0];
        for i in 1..groups.len() {
            let island = &groups[i];
            let start = island[0];
            let end = main_group[0];
            
            let path = a_star_search(start, end, map);
            if path.success {
                for tile_idx in path.steps {
                    map.tiles[tile_idx] = TileType::Floor;
                }
            }
        }
    }

    // 3. Puntos de Interés Dijkstra (Spec-12)
    // Se calcula la distancia desde el centro
    let dm = DijkstraMap::new(map.width, map.height, &[start_idx], map, 100.0);
    let mut furthest_idx = start_idx;
    let mut max_dist = 0.0;

    for (i, &dist) in dm.map.iter().enumerate() {
        if dist < 1e10 && dist > max_dist {
            max_dist = dist;
            furthest_idx = i;
        }
    }

    // Marcar punto de interés (ej. Lab en el punto más alejado)
    map.tiles[furthest_idx] = TileType::Floor;
    map.interest_points.push(furthest_idx);
    
    map.update_map_metadata(None);
}

fn count_neighbors_static(tiles: &[TileType], width: i32, height: i32, x: i32, y: i32) -> usize {
    let mut neighbors = 0;
    for iy in -1..=1 {
        for ix in -1..=1 {
            if ix == 0 && iy == 0 { continue; }
            let nx = (x + ix).rem_euclid(width);
            let ny = (y + iy).rem_euclid(height);
            let idx = (ny * width + nx) as usize;
            if tiles[idx] == TileType::Wall { neighbors += 1; }
        }
    }
    neighbors
}

pub fn generate_caverns(width: i32, height: i32, seed: u64) -> Map {
    let mut map = Map::new(width, height);
    for i in 0..11 {
        generate_caverns_step(&mut map, i, seed);
    }
    ensure_connectivity_step(&mut map);
    map
}
