use super::map::{Map, TileType};
use bracket_lib::prelude::*;
use rayon::prelude::*;
use noise::{NoiseFn, Simplex};

/// Generador Planetario Sin Costuras (Toroidal)
pub fn build_planet(seed: u64) -> Map {
    let mut map = Map::new_planet();
    
    apply_global_noise(&mut map, seed);
    for _ in 0..5 {
        apply_toroidal_cellular_automata(&mut map);
    }
    apply_toroidal_drunkards_walk(&mut map, seed);
    
    map
}

/// 1. Ruido Global Continuo: Biomas base sin costuras
fn apply_global_noise(map: &mut Map, seed: u64) {
    let width = map.width;
    
    let simplex_moisture = Simplex::new(seed as u32);
    let simplex_temp = Simplex::new((seed >> 32) as u32);
    let simplex_elevation = Simplex::new((seed ^ 0xDEADBEEF) as u32);

    map.tiles.par_iter_mut().enumerate().for_each(|(idx, tile)| {
        let x = idx as i32 % width;
        let y = idx as i32 / width;

        // Para evitar distorsión en los bordes de un mapa 2D, lo proyectamos a un cilindro/toroide 
        // usando ángulos, pero para mantener rendimiento en Celeron usamos coordenadas 
        // escaladas (OpenSimplex maneja bien la continuidad si la frecuencia es baja, aunque un 
        // mapeo esférico real requeriría sin/cos. Por simplicidad planetaria, usamos coordenadas directas).
        let nx = x as f64 / 40.0;
        let ny = y as f64 / 40.0;

        let elevation = simplex_elevation.get([nx, ny]);
        let moisture = simplex_moisture.get([nx, ny]);
        let temp = simplex_temp.get([nx, ny]);

        if elevation < -0.4 {
            *tile = TileType::DeepWater;
        } else if elevation < -0.1 {
            *tile = TileType::ShallowWater;
        } else if elevation > 0.6 {
            *tile = TileType::Wall;
        } else {
            if moisture > 0.3 {
                *tile = TileType::MuddyFloor;
            } else if temp < -0.2 {
                *tile = TileType::StonyFloor;
            } else {
                *tile = TileType::Floor;
            }
        }
        
        // Agregar "semillas" de muros para autómatas celulares en la tierra
        if *tile != TileType::DeepWater && *tile != TileType::ShallowWater && *tile != TileType::Wall {
            let mut rng = RandomNumberGenerator::seeded(seed + idx as u64);
            if rng.range(0, 100) < 45 {
                *tile = TileType::Wall;
            }
        }
    });
}

/// 2. Autómatas Celulares Toroidales
fn apply_toroidal_cellular_automata(map: &mut Map) {
    let mut new_tiles = map.tiles.clone();
    let width = map.width;
    let height = map.height;

    new_tiles.par_iter_mut().enumerate().for_each(|(idx, tile)| {
        let x = idx as i32 % width;
        let y = idx as i32 / width;

        // No procesar agua
        if *tile == TileType::DeepWater || *tile == TileType::ShallowWater {
            return;
        }

        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if ix == 0 && iy == 0 { continue; }
                let nx = (x + ix).rem_euclid(width);
                let ny = (y + iy).rem_euclid(height);
                let n_idx = (ny * width + nx) as usize;
                if map.tiles[n_idx] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        if neighbors >= 5 {
            *tile = TileType::Wall;
        } else if neighbors < 3 {
            // Revertir a suelo original si no es muro (simplificado)
            *tile = TileType::Floor;
        }
    });

    map.tiles = new_tiles;
}

/// 3. Drunkard's Walk Planetario (Excavador Toroidal)
fn apply_toroidal_drunkards_walk(map: &mut Map, seed: u64) {
    let width = map.width;
    let height = map.height;
    let mut rng = RandomNumberGenerator::seeded(seed);
    
    // Queremos asegurar que gran parte de los muros terrestres sean cavernas interconectadas
    let target_floor = (width * height) as f32 * 0.40;
    let mut floor_count = map.tiles.iter().filter(|&&t| t == TileType::Floor || t == TileType::MuddyFloor || t == TileType::StonyFloor).count();

    if (floor_count as f32) < target_floor {
        let mut drunk_x = rng.range(0, width);
        let mut drunk_y = rng.range(0, height);
        let mut lifetime = 10000; // Vidas largas para crear túneles expansivos

        while lifetime > 0 {
            let idx = map.xy_idx(drunk_x, drunk_y);
            
            // Si es un muro, lo rompemos
            if map.tiles[idx] == TileType::Wall {
                map.tiles[idx] = TileType::Floor;
                floor_count += 1;
            }

            // Movimiento puro sin límites
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
}
