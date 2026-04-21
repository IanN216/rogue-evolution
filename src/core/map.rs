use crate::core::world_map::{PLANET_TILE_WIDTH, PLANET_TILE_HEIGHT};
use bracket_lib::prelude::*;

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
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y.rem_euclid(self.height) as usize * self.width as usize) + x.rem_euclid(self.width) as usize
    }

    pub fn new_planet() -> Map {
        let width = PLANET_TILE_WIDTH;
        let height = PLANET_TILE_HEIGHT;
        Map {
            tiles: vec![TileType::DeepWater; (width * height) as usize],
            width,
            height,
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall || self.tiles[idx] == TileType::Mountain
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
