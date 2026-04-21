use crate::core::map::TileType;
use crate::core::world_map::CHUNK_SIZE;

pub struct Prefab {
    pub width: i32,
    pub height: i32,
    pub data: Vec<TileType>,
}

pub fn get_ruins_prefab() -> Prefab {
    let w = 5;
    let h = 5;
    let mut data = vec![TileType::StonyFloor; (w * h) as usize];
    
    // Crear un marco de muros
    for x in 0..w {
        data[x as usize] = TileType::Wall;
        data[((h-1)*w + x) as usize] = TileType::Wall;
    }
    for y in 0..h {
        data[(y*w) as usize] = TileType::Wall;
        data[(y*w + (w-1)) as usize] = TileType::Wall;
    }
    // Entrada
    data[(2*w) as usize] = TileType::StonyFloor;

    Prefab { width: w, height: h, data }
}

pub fn stamp_prefab(tiles: &mut [TileType], prefab: &Prefab, x: i32, y: i32) {
    for py in 0..prefab.height {
        for px in 0..prefab.width {
            let tx = x + px;
            let ty = y + py;
            if tx >= 0 && tx < CHUNK_SIZE && ty >= 0 && ty < CHUNK_SIZE {
                let idx = (ty * CHUNK_SIZE + tx) as usize;
                let p_idx = (py * prefab.width + px) as usize;
                tiles[idx] = prefab.data[p_idx];
            }
        }
    }
}
