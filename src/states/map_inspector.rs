use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::core::map::{TileType, Map};

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, zoom: f32, cursor: (i32, i32)) -> Option<RunState> {
    ctx.set_active_console(0);
    ctx.cls();

    let (mut cursor_x, mut cursor_y) = cursor;
    let mut current_zoom = zoom;

    // Viewport calculation based on zoom
    let map = &world_manager.world_map.map; 
    let view_w = (80.0 / current_zoom) as i32;
    let view_h = (50.0 / current_zoom) as i32;
    
    let start_x = cursor_x - view_w / 2;
    let start_y = cursor_y - view_h / 2;

    for y in 0..view_h {
        for x in 0..view_w {
            let map_x = start_x + x;
            let map_y = start_y + y;

            if map_x >= 0 && map_x < map.width && map_y >= 0 && map_y < map.height {
                let idx = map.xy_idx(map_x, map_y);
                let tile = map.tiles[idx];
                let mut color = match tile {
                    TileType::Floor => RGB::named(GRAY),
                    TileType::Wall => RGB::named(WHITE),
                };

                // Diagnostics
                if tile == TileType::Floor && is_isolated(map, map_x, map_y) {
                    color = RGB::named(RED);
                } else if tile == TileType::Wall && is_thin_wall(map, map_x, map_y) {
                    color = RGB::named(YELLOW);
                }

                if map_x == cursor_x && map_y == cursor_y {
                    color = RGB::named(CYAN);
                }

                ctx.set(x, y, color, RGB::named(BLACK), to_cp437(match tile {
                    TileType::Floor => '.',
                    TileType::Wall => '#',
                }));
            }
        }
    }

    // HUD
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print(1, 1, format!("Inspector - Zoom: {:.1}x", current_zoom));
    ctx.print(1, 2, format!("Cursor: ({}, {})", cursor_x, cursor_y));
    ctx.print(1, 48, "[+/-] Zoom | [Arrows] Move | [Esc] Exit");

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Escape => return Some(RunState::MainMenu { selection: crate::states::MainMenuSelection::NewGame }),
            VirtualKeyCode::Plus | VirtualKeyCode::NumpadAdd => current_zoom += 0.1,
            VirtualKeyCode::Minus | VirtualKeyCode::NumpadSubtract => if current_zoom > 0.5 { current_zoom -= 0.1 },
            VirtualKeyCode::Up => cursor_y -= 1,
            VirtualKeyCode::Down => cursor_y += 1,
            VirtualKeyCode::Left => cursor_x -= 1,
            VirtualKeyCode::Right => cursor_x += 1,
            _ => {}
        }
    }

    Some(RunState::MapInspector { zoom: current_zoom, cursor: (cursor_x, cursor_y) })
}

fn is_isolated(map: &Map, x: i32, y: i32) -> bool {
    let mut neighbors = 0;
    for iy in -1..=1 {
        for ix in -1..=1 {
            if ix == 0 && iy == 0 { continue; }
            let nx = x + ix;
            let ny = y + iy;
            if nx >= 0 && nx < map.width && ny >= 0 && ny < map.height {
                if map.tiles[map.xy_idx(nx, ny)] == TileType::Floor { neighbors += 1; }
            }
        }
    }
    neighbors == 0
}

fn is_thin_wall(map: &Map, x: i32, y: i32) -> bool {
    // A wall is "thin" if it's surrounded by floors in a way that looks like a single-tile barrier
    let mut floor_count = 0;
    for iy in -1..=1 {
        for ix in -1..=1 {
            if ix == 0 && iy == 0 { continue; }
            let nx = x + ix;
            let ny = y + iy;
            if nx >= 0 && nx < map.width && ny >= 0 && ny < map.height {
                if map.tiles[map.xy_idx(nx, ny)] == TileType::Floor { floor_count += 1; }
            }
        }
    }
    floor_count >= 6
}
