use bracket_lib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseStats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightSource {
    pub range: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct InCombat;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlocksTile;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metabolism {
    pub hunger: f32,
    pub max_hunger: f32,
    pub hunger_rate: f32,
}
