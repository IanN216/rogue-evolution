use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Humanoid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub level: u32,
    pub xp: u32,
    pub next_level_xp: u32,
}

impl Experience {
    pub fn new() -> Self {
        Self {
            level: 1,
            xp: 0,
            next_level_xp: 100,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub hp_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityRegistry {
    pub abilities: Vec<Ability>,
}

impl AbilityRegistry {
    pub fn new() -> Self {
        Self {
            abilities: Vec::new(),
        }
    }
}
