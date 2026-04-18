use std::fs::{File, self};
use std::io::{Write, Read};
use std::path::Path;
use crate::core::world_map::RegionData;
use bincode;

pub fn get_region_filename(x: i32, y: i32) -> String {
    format!("region_{}_{}.bin", x, y)
}

pub fn save_region_async(region: RegionData) {
    std::thread::spawn(move || {
        if let Err(e) = save_region(&region) {
            eprintln!("Error saving region {},{}: {}", region.x, region.y, e);
        }
    });
}

const MAGIC_NUMBER: [u8; 4] = [0x52, 0x4F, 0x47, 0x55]; // "ROGU"
const VERSION: u8 = 1;

pub fn save_region(region: &RegionData) -> Result<(), Box<dyn std::error::Error>> {
    let filename = get_region_filename(region.x, region.y);
    let path = Path::new("saves").join(filename);
    
    if !Path::new("saves").exists() {
        fs::create_dir_all("saves")?;
    }

    let encoded: Vec<u8> = bincode::serialize(region)?;
    
    // Simple checksum: XOR of all bytes (very fast for Celeron)
    let mut checksum: u8 = 0;
    for &byte in &encoded {
        checksum ^= byte;
    }

    let mut file = File::create(path)?;
    file.write_all(&MAGIC_NUMBER)?; // Magic Number
    file.write_all(&[VERSION])?;    // Version
    file.write_all(&[checksum])?;   // Checksum
    file.write_all(&encoded)?;
    
    Ok(())
}

pub fn load_region(x: i32, y: i32) -> Result<RegionData, Box<dyn std::error::Error>> {
    let filename = get_region_filename(x, y);
    let path = Path::new("saves").join(filename);
    
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 6 { // Magic (4) + Version (1) + Checksum (1)
        return Err("File too small or invalid header".into());
    }

    if data[0..4] != MAGIC_NUMBER {
        return Err("Invalid Magic Number - Not a Rogue-Evolution save".into());
    }

    if data[4] != VERSION {
        return Err(format!("Incompatible version: expected {}, found {}", VERSION, data[4]).into());
    }

    let stored_checksum = data[5];
    let encoded = &data[6..];

    let mut checksum: u8 = 0;
    for &byte in encoded {
        checksum ^= byte;
    }

    if checksum != stored_checksum {
        return Err("Checksum mismatch - Corrupted region file".into());
    }

    let region: RegionData = bincode::deserialize(encoded)?;
    Ok(region)
}
