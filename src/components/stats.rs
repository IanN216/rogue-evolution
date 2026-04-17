use bracket_lib::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BaseStats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewshed {
    pub range: i32,
    pub dirty: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InCombat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metabolism {
    pub hunger: f32,
    pub max_hunger: f32,
    pub hunger_rate: f32,
}
