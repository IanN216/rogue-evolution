mod core;

use bracket_lib::prelude::*;
use crate::core::map::{Map, TileType};
use crate::core::map_gen::build_planet;

struct State {
    map: Map,
    camera_x: i32,
    camera_y: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        // 1. Detección de Entrada (Movimiento de Cámara Toroidal)
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left | VirtualKeyCode::A => self.camera_x -= 2,
                VirtualKeyCode::Right | VirtualKeyCode::D => self.camera_x += 2,
                VirtualKeyCode::Up | VirtualKeyCode::W => self.camera_y -= 2,
                VirtualKeyCode::Down | VirtualKeyCode::S => self.camera_y += 2,
                VirtualKeyCode::R => {
                    let seed = rand::random::<u64>();
                    self.map = build_planet(seed);
                }
                VirtualKeyCode::Escape => ctx.quit(),
                _ => {}
            }
        }

        self.camera_x = self.camera_x.rem_euclid(self.map.width);
        self.camera_y = self.camera_y.rem_euclid(self.map.height);

        // 2. Renderizado Toroidal
        let screen_width = 80;
        let screen_height = 50;
        
        let offset_x = self.camera_x - (screen_width / 2);
        let offset_y = self.camera_y - (screen_height / 2);

        for y in 0..screen_height {
            for x in 0..screen_width {
                let world_x = (x + offset_x).rem_euclid(self.map.width);
                let world_y = (y + offset_y).rem_euclid(self.map.height);
                
                let idx = self.map.xy_idx(world_x, world_y);
                let tile = self.map.tiles[idx];

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
                };
                
                ctx.set(x, y, fg, bg, glyph);
            }
        }

        // HUD
        ctx.draw_box(0, 0, 79, 2, RGB::named(WHITE), RGB::named(BLACK));
        ctx.print(2, 1, &format!("Planeta: {}x{} | Cámara: ({}, {})", self.map.width, self.map.height, self.camera_x, self.camera_y));
        ctx.print(40, 1, "WASD: Mover | R: Regenerar | ESC: Salir");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rogue-Evolution: Planet Engine (Advanced Noise)")
        .with_fps_cap(60.0)
        .build()?;

    let seed = 12345;
    let gs = State {
        map: build_planet(seed),
        camera_x: 0,
        camera_y: 0,
    };
    
    main_loop(context, gs)
}
