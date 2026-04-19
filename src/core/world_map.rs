use serde::{Deserialize, Serialize};
use crate::core::map::TileType;
use crate::components::stats::{Position, Renderable, BaseStats, Viewshed, Metabolism};
use crate::components::genetics::Genetics;
use crate::components::identity::Identity;
use crate::components::kingdom::KingdomMember;
use std::collections::{HashMap, HashSet};

pub const PARASANGA_SIZE: i32 = 64;

use crate::components::progression::{Experience, AbilityRegistry};
use crate::components::items::{Item, Weapon};

#[derive(Serialize, Deserialize, Clone)]
pub struct EntitySnapshot {
    pub position: Position,
    pub renderable: Option<Renderable>,
    pub base_stats: Option<BaseStats>,
    pub viewshed: Option<Viewshed>,
    pub genetics: Option<Genetics>,
    pub identity: Option<Identity>,
    pub kingdom_member: Option<KingdomMember>,
    pub metabolism: Option<Metabolism>,
    pub experience: Option<Experience>,
    pub abilities: Option<AbilityRegistry>,
    pub is_humanoid: bool,
    pub item: Option<Item>,
    pub weapon: Option<Weapon>,
    pub is_blighted: bool,
    pub is_infection_source: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegionData {
    pub x: i32,
    pub y: i32,
    pub tiles: Vec<TileType>,
    pub entities: Vec<EntitySnapshot>,
}

use crate::core::map::Map;

pub struct WorldMap {
    pub map: Map,
    pub regions: HashMap<(i32, i32), RegionData>,
    pub loaded_regions: HashSet<(i32, i32)>,
}

impl WorldMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            regions: HashMap::new(),
            loaded_regions: HashSet::new(),
        }
    }

    pub fn get_region_coords(x: i32, y: i32) -> (i32, i32) {
        (x / PARASANGA_SIZE, y / PARASANGA_SIZE)
    }
}
