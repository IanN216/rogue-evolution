use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DisplayMode {
    Windowed80x50,
    FullscreenNative170x48,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Settings {
    pub display_mode: DisplayMode,
    pub fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            display_mode: DisplayMode::Windowed80x50,
            fullscreen: false,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        if !Path::new("settings.bin").exists() {
            return Self::default();
        }

        let mut file = match File::open("settings.bin") {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return Self::default();
        }

        match bincode::deserialize(&buffer) {
            Ok(s) => s,
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let encoded: Vec<u8> = bincode::serialize(self).expect("Failed to serialize settings");
        let mut file = File::create("settings.bin").expect("Failed to create settings file");
        file.write_all(&encoded).expect("Failed to write settings file");
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        match self.display_mode {
            DisplayMode::Windowed80x50 => (80, 50),
            DisplayMode::FullscreenNative170x48 => (170, 48),
        }
    }
}
