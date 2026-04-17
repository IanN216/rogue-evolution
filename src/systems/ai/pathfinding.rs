use crate::core::map::Map;
use bracket_lib::prelude::*;

pub struct NavigationMap {
    pub map: DijkstraMap,
}

impl NavigationMap {
    pub fn new(width: i32, height: i32, start_points: &[usize], map: &Map) -> Self {
        let mut dm = DijkstraMap::new(width, height, start_points, map, 100.0);
        DijkstraMap::build(&mut dm, start_points, map);
        Self { map: dm }
    }

    pub fn get_next_step(&self, idx: usize, map: &Map) -> Option<usize> {
        let exits = map.get_available_exits(idx);
        let mut best_idx = None;
        let mut best_dist = self.map.map[idx];

        for (exit_idx, _) in exits {
            let dist = self.map.map[exit_idx];
            if dist < best_dist {
                best_dist = dist;
                best_idx = Some(exit_idx);
            }
        }
        best_idx
    }
}
