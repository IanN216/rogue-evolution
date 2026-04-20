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
        (y as usize * self.width as usize) + x as usize
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
        if x < 0 || x >= self.width || y < 0 || y >= self.height { return false; }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    /// Actualiza toda la metadata del mapa (muros y entidades bloqueantes) en un solo ciclo de vida.
    /// Esto elimina la redundancia y optimiza el uso del caché L1/L2 del Celeron.
    pub fn update_map_metadata(&mut self, world: Option<&World>) {
        let count = self.tiles.len();
        
        // Sincronización robusta de dimensiones para evitar pánicos tras carga de regiones
        if self.blocked.len() != count { self.blocked.resize(count, false); }
        if self.revealed_tiles.len() != count { self.revealed_tiles.resize(count, false); }
        if self.visible_tiles.len() != count { self.visible_tiles.resize(count, false); }

        // Fase 1: Sincronizar bloqueos basados en la topología estática (muros)
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }

        // Fase 2: Integrar bloqueos dinámicos desde el ECS (entidades con BlocksTile)
        if let Some(world) = world {
            for (_entity, (pos, _)) in world.query::<(&Position, &BlocksTile)>().iter() {
                let idx = self.xy_idx(pos.x, pos.y);
                if idx < self.blocked.len() {
                    self.blocked[idx] = true;
                }
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
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + w, 1.0)) };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}
