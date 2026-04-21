use bracket_lib::prelude::*;
use serde::{Deserialize, Serialize};
use hecs::World;
use crate::components::stats::{Position, BlocksTile};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TileType {
    Wall,
    Floor,
    StonyFloor,
    MuddyFloor,
}

use crate::core::world_map::{PARASANGA_SIZE, WORLD_WIDTH_REGIONS, WORLD_HEIGHT_REGIONS};

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub interest_points: Vec<usize>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y.rem_euclid(self.height) as usize * self.width as usize) + x.rem_euclid(self.width) as usize
    }

    /// Crea un nuevo mapa del tamaño completo del planeta
    pub fn new_planet() -> Map {
        let width = PARASANGA_SIZE * WORLD_WIDTH_REGIONS;
        let height = PARASANGA_SIZE * WORLD_HEIGHT_REGIONS;
        let map_tile_count = (width * height) as usize;
        Map {
            tiles: vec![TileType::Wall; map_tile_count],
            width,
            height,
            revealed_tiles: vec![false; map_tile_count],
            visible_tiles: vec![false; map_tile_count],
            blocked: vec![false; map_tile_count],
            interest_points: Vec::new(),
        }
    }

    pub fn new(width: i32, height: i32) -> Map {
        let map_tile_count = (width * height) as usize;
        Map {
            tiles: vec![TileType::Wall; map_tile_count],
            width,
            height,
            revealed_tiles: vec![false; map_tile_count],
            visible_tiles: vec![false; map_tile_count],
            blocked: vec![false; map_tile_count],
            interest_points: Vec::new(),
        }
    }

    pub fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    /// Actualiza toda la metadata del mapa (muros y entidades bloqueantes) en un solo ciclo de vida.
    /// Esto elimina la redundancia y optimiza el uso del caché L1/L2 del Celeron.
    pub fn update_map_metadata(&mut self, world: Option<&World>) {
        let count = self.tiles.len();
        
        // Sincronización robusta de dimensiones
        if self.blocked.len() != count { self.blocked.resize(count, false); }
        if self.revealed_tiles.len() != count { self.revealed_tiles.resize(count, false); }
        if self.visible_tiles.len() != count { self.visible_tiles.resize(count, false); }

        // Fase 1: Sincronizar bloqueos basados en la topología estática (muros)
        // Optimizamos usando rayon para el Celeron dual-core
        use rayon::prelude::*;
        self.blocked.par_iter_mut().enumerate().for_each(|(i, b)| {
            *b = self.tiles[i] == TileType::Wall;
        });

        // Fase 2: Integrar bloqueos dinámicos desde el ECS (entidades con BlocksTile)
        if let Some(world) = world {
            for (_entity, (pos, _)) in world.query::<(&Position, &BlocksTile)>().iter() {
                // Aplicamos rem_euclid por seguridad si el sistema de unificación no ha corrido
                let idx = self.xy_idx(pos.x, pos.y);
                self.blocked[idx] = true;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;

        // Cardinal directions con wrap-around
        let neighbors = [
            (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)
        ];

        for (nx, ny) in neighbors {
            if self.is_exit_valid(nx, ny) {
                exits.push((self.xy_idx(nx, ny), 1.0));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let x1 = (idx1 as i32 % self.width) as f32;
        let y1 = (idx1 as i32 / self.width) as f32;
        let x2 = (idx2 as i32 % self.width) as f32;
        let y2 = (idx2 as i32 / self.width) as f32;

        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();

        let dx = dx.min(self.width as f32 - dx);
        let dy = dy.min(self.height as f32 - dy);

        (dx * dx + dy * dy).sqrt()
    }
}
