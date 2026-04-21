mod core;

use bracket_lib::prelude::*;
use crate::core::map::{Map, TileType};
use crate::core::map_gen::{build_planet, generate_chunk};
use crate::core::world_map::{ChunkKey, VIEW_DISTANCE, CHUNK_SIZE, River, generate_global_rivers};
use noise::{NoiseFn, Simplex};

struct Position { x: i32, y: i32 }
struct ChunkEntity { key: ChunkKey }
struct Flora { name: String }

struct State {
    map: Map,
    camera_x: i32,
    camera_y: i32,
    seed: u64,
    global_rivers: Vec<River>,
}

impl State {
    fn populate_chunk_entities(&mut self, key: ChunkKey) {
        let chunk_data = self.map.chunks.get(&key).unwrap();
        let n_flora = Simplex::new((self.seed ^ 0xF0F0) as u32);

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let idx = (y * CHUNK_SIZE + x) as usize;
                let tile = chunk_data[idx];
                
                if tile == TileType::Forest || tile == TileType::Jungle || tile == TileType::Grass {
                    let world_x = key.x * CHUNK_SIZE + x;
                    let world_y = key.y * CHUNK_SIZE + y;
                    
                    let noise_val = n_flora.get([world_x as f64 * 0.4, world_y as f64 * 0.4]);
                    if noise_val > 0.4 {
                        self.map.world.spawn((
                            Flora { name: "Tree".to_string() },
                            Position { x: world_x, y: world_y },
                            ChunkEntity { key }
                        ));
                    }
                }
            }
        }
    }

    fn update_chunks(&mut self) {
        let center_key = ChunkKey::from_world_coords(self.camera_x, self.camera_y);
        let mut loaded_keys = std::collections::HashSet::new();

        for y in -VIEW_DISTANCE..=VIEW_DISTANCE {
            for x in -VIEW_DISTANCE..=VIEW_DISTANCE {
                let key = ChunkKey::new(center_key.x + x, center_key.y + y);
                loaded_keys.insert(key);
                
                if !self.map.chunks.contains_key(&key) {
                    let chunk_data = generate_chunk(key, self.seed, &self.global_rivers);
                    self.map.chunks.insert(key, chunk_data);
                    self.populate_chunk_entities(key);
                }
            }
        }

        let mut to_despawn = Vec::new();
        for (entity, (_, chunk_ent)) in self.map.world.query::<(&Position, &ChunkEntity)>().iter() {
            if !loaded_keys.contains(&chunk_ent.key) {
                to_despawn.push(entity);
            }
        }
        for entity in to_despawn {
            let _ = self.map.world.despawn(entity);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left | VirtualKeyCode::A => self.camera_x -= 2,
                VirtualKeyCode::Right | VirtualKeyCode::D => self.camera_x += 2,
                VirtualKeyCode::Up | VirtualKeyCode::W => self.camera_y -= 2,
                VirtualKeyCode::Down | VirtualKeyCode::S => self.camera_y += 2,
                VirtualKeyCode::R => {
                    self.seed = rand::random::<u64>();
                    self.global_rivers = generate_global_rivers(self.seed);
                    self.map = build_planet(self.seed);
                }
                VirtualKeyCode::Escape => ctx.quit(),
                _ => {}
            }
        }

        self.camera_x = self.camera_x.rem_euclid(self.map.width);
        self.camera_y = self.camera_y.rem_euclid(self.map.height);
        self.update_chunks();

        let screen_width = 80;
        let screen_height = 50;
        let offset_x = self.camera_x - (screen_width / 2);
        let offset_y = self.camera_y - (screen_height / 2);

        for y in 0..screen_height {
            for x in 0..screen_width {
                let world_x = (x + offset_x).rem_euclid(self.map.width);
                let world_y = (y + offset_y).rem_euclid(self.map.height);
                let tile = self.map.get_tile(world_x, world_y);

                let (glyph, fg, bg) = match tile {
                    TileType::DeepWater => (to_cp437('~'), RGB::named(BLUE), RGB::named(DARK_BLUE)),
                    TileType::ShallowWater => (to_cp437('~'), RGB::named(CYAN), RGB::named(BLUE)),
                    TileType::Sand => (to_cp437('.'), RGB::named(YELLOW), RGB::named(BLACK)),
                    TileType::Grass => (to_cp437('"'), RGB::named(GREEN), RGB::named(BLACK)),
                    TileType::Forest => (to_cp437('♣'), RGB::named(FOREST_GREEN), RGB::named(BLACK)),
                    TileType::Mountain => (to_cp437('▲'), RGB::named(BROWN1), RGB::named(BLACK)),
                    TileType::Snow => (to_cp437('*'), RGB::named(WHITE), RGB::named(BLACK)),
                    TileType::Wall => (to_cp437('#'), RGB::named(GRAY), RGB::named(BLACK)),
                    TileType::StonyFloor => (to_cp437('.'), RGB::named(GRAY), RGB::named(BLACK)),
                    TileType::MuddyFloor => (to_cp437('~'), RGB::named(CHOCOLATE), RGB::named(BLACK)),
                    TileType::Tundra => (to_cp437('.'), RGB::named(SNOW), RGB::named(BLACK)),
                    TileType::Jungle => (to_cp437('♣'), RGB::named(DARK_GREEN), RGB::named(BLACK)),
                    TileType::Savanna => (to_cp437('"'), RGB::named(GOLDENROD), RGB::named(BLACK)),
                    TileType::Desert => (to_cp437('.'), RGB::named(KHAKI), RGB::named(BLACK)),
                };
                ctx.set(x, y, fg, bg, glyph);
            }
        }

        ctx.draw_box(0, 0, 79, 2, RGB::named(WHITE), RGB::named(BLACK));
        ctx.print(2, 1, &format!("Planeta: {}x{} | Chunks: {}", self.map.width, self.map.height, self.map.chunks.len()));
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rogue-Evolution: Planet Engine")
        .with_fps_cap(60.0)
        .build()?;

    let seed = 12345;
    let rivers = generate_global_rivers(seed);
    let mut gs = State {
        map: build_planet(seed),
        camera_x: 0,
        camera_y: 0,
        seed,
        global_rivers: rivers,
    };
    gs.update_chunks();
    main_loop(context, gs)
}
