use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MaterialType {
    Iron,
    Glass,
    Wood,
    Organic,
}

impl MaterialType {
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            MaterialType::Iron => 1.0,
            MaterialType::Glass => 1.5,
            MaterialType::Wood => 0.7,
            MaterialType::Organic => 0.9,
        }
    }

    pub fn fragility_level(&self) -> f32 {
        match self {
            MaterialType::Iron => 0.1,
            MaterialType::Glass => 0.8,
            MaterialType::Wood => 0.3,
            MaterialType::Organic => 0.5,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub material: MaterialType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub base_damage: i32,
}

impl Weapon {
    pub fn get_damage(&self, material: &MaterialType) -> i32 {
        (self.base_damage as f32 * material.damage_multiplier()) as i32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blighted;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfectionSource;
