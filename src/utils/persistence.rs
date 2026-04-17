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
    file.write_all(&[checksum])?; // Write checksum first
    file.write_all(&encoded)?;
    
    Ok(())
}

pub fn load_region(x: i32, y: i32) -> Result<RegionData, Box<dyn std::error::Error>> {
    let filename = get_region_filename(x, y);
    let path = Path::new("saves").join(filename);
    
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 1 {
        return Err("File too small".into());
    }

    let stored_checksum = data[0];
    let encoded = &data[1..];

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
