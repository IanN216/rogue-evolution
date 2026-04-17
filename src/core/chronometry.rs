use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeState {
    pub ticks: u64,
}

impl TimeState {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }

    pub fn update(&mut self) {
        self.ticks += 1;
    }

    pub fn get_time_of_day(&self) -> TimeOfDay {
        let hour = (self.ticks / 100) % 24;
        match hour {
            5..=6 => TimeOfDay::Dawn,
            7..=17 => TimeOfDay::Day,
            18..=19 => TimeOfDay::Dusk,
            _ => TimeOfDay::Night,
        }
    }

    pub fn get_visibility_radius(&self) -> i32 {
        match self.get_time_of_day() {
            TimeOfDay::Day => 30,
            TimeOfDay::Dawn | TimeOfDay::Dusk => 15,
            TimeOfDay::Night => 5,
        }
    }
}
