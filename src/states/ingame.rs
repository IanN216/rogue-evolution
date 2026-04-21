use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::core::chronometry::TimeState;
use crate::components::stats::{Position, Renderable, Viewshed, BaseStats};
use crate::components::identity::Identity;
use crate::systems::sensory::fov::process_fov;
use crate::systems::ai::swarm::process_swarm_ai;
use crate::core::map::TileType;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, time_state: &mut TimeState, current_state: RunState) -> Option<RunState> {
    let mut player_pos = Position { x: 0, y: 0 };
    let mut player_viewshed = None;
    
    // Obtener datos del jugador una sola vez para renderizado e IA
    for (_entity, (pos, viewshed, id)) in world_manager.world.query::<(&Position, &Viewshed, &Identity)>().iter() {
        if id.name == "Hero" {
            player_pos = *pos;
            player_viewshed = Some(viewshed.clone());
        }
    }

    // 1. Renderizado con Cámara Dinámica
    ctx.set_active_console(2); // Limpieza de capa UI para evitar ghosting
    ctx.cls();
    render(ctx, world_manager, player_pos, &player_viewshed);

    // 2. Lógica según el estado de turno
    match current_state {
        RunState::InGame => {
            // Procesar FOV inicial o si está sucio para evitar pantalla negra al empezar
            if let Some(vs) = &player_viewshed {
                if vs.dirty {
                    process_fov(&mut world_manager.world, &world_manager.world_map.map, time_state);
                }
            }

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::Numpad4 => return try_move_player(-1, 0, world_manager),
                    VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::Numpad6 => return try_move_player(1, 0, world_manager),
                    VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::Numpad8 => return try_move_player(0, -1, world_manager),
                    VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::Numpad2 => return try_move_player(0, 1, world_manager),
                    VirtualKeyCode::M => return Some(RunState::MainMenu { selection: crate::states::MainMenuSelection::NewGame }),
                    VirtualKeyCode::Escape => return Some(RunState::PauseMenu { selection: 0 }),
                    _ => {}
                }
            }
            None
        }
        RunState::PlayerTurn => {
            process_fov(&mut world_manager.world, &world_manager.world_map.map, time_state);
            Some(RunState::MonsterTurn)
        }
        RunState::MonsterTurn => {
            time_state.update();
            
            // Procesar Streaming de Regiones (Spec-5: Parasangas)
            world_manager.stream_regions(player_pos);

            process_swarm_ai(&mut world_manager.world, &world_manager.world_map.map);
            
            // Sistemas Biológicos (Celeron Optimized)
            crate::systems::biology::metabolism::process_metabolism(&mut world_manager.world);
            crate::systems::biology::infection::process_infection(
                &mut world_manager.world, 
                world_manager.world_map.map.width, 
                world_manager.world_map.map.height
            );
            crate::systems::biology::evolution::process_evolution(&mut world_manager.world, time_state.ticks);

            // Unificación de Coordenadas Toroidales (Arquitectura Senior)
            crate::systems::coordinate_unification(&mut world_manager.world);

            Some(RunState::InGame)
        }
        _ => None
    }
}

fn try_move_player(dx: i32, dy: i32, world_manager: &mut WorldManager) -> Option<RunState> {
    let query = world_manager.world.query_mut::<(&mut Position, &mut Viewshed, &Identity)>();
    for (_entity, (pos, viewshed, id)) in query {
        if id.name == "Hero" {
            let new_x = (pos.x + dx).rem_euclid(world_manager.world_map.map.width);
            let new_y = (pos.y + dy).rem_euclid(world_manager.world_map.map.height);
            
            // Validar colisión con el mapa real
            if world_manager.world_map.map.is_exit_valid(new_x, new_y) {
                pos.x = new_x;
                pos.y = new_y;
                viewshed.dirty = true;
                return Some(RunState::PlayerTurn);
            }
        }
    }
    None
}

fn render(ctx: &mut BTerm, world_manager: &mut WorldManager, player_pos: Position, viewshed: &Option<Viewshed>) {
    let (screen_width, screen_height) = (80, 40);
    let offset_x = player_pos.x - (screen_width / 2);
    let offset_y = player_pos.y - (screen_height / 2);

    // Capa 0: Mapa
    ctx.set_active_console(0);
    ctx.cls();
    
    let mut visible_tiles = std::collections::HashSet::new();
    if let Some(vs) = viewshed {
        for pt in vs.visible_tiles.iter() {
            visible_tiles.insert((pt.x, pt.y));
        }
    }

    for y in 0..screen_height {
        for x in 0..screen_width {
            let world_x = (x + offset_x).rem_euclid(world_manager.world_map.map.width);
            let world_y = (y + offset_y).rem_euclid(world_manager.world_map.map.height);
            
            if visible_tiles.contains(&(world_x, world_y)) {
                let idx = world_manager.world_map.map.xy_idx(world_x, world_y);
                if let Some(tile) = world_manager.world_map.map.tiles.get(idx) {
                    let (glyph, fg) = match tile {
                        TileType::Floor => (to_cp437('.'), RGB::named(DARK_GRAY)),
                        TileType::StonyFloor => (to_cp437('.'), RGB::named(GRAY)),
                        TileType::MuddyFloor => (to_cp437('~'), RGB::named(CHOCOLATE)),
                        TileType::Wall => (to_cp437('#'), RGB::named(GREEN)),
                    };
                    ctx.set(x, y, fg, RGB::named(BLACK), glyph);
                }
            }
        }
    }

    // Capa 1: Entidades
    ctx.set_active_console(1);
    ctx.cls();
    for (_entity, (pos, render, id)) in world_manager.world.query::<(&Position, &Renderable, Option<&Identity>)>().iter() {
        // Calculamos la posición relativa en pantalla considerando el wrap-around toroidal
        let mut sx = pos.x - offset_x;
        let mut sy = pos.y - offset_y;

        // Si la entidad está muy lejos del centro de la cámara por el wrap, la re-centramos
        if sx < -screen_width / 2 { sx += world_manager.world_map.map.width; }
        if sx >= world_manager.world_map.map.width - screen_width / 2 { sx -= world_manager.world_map.map.width; }
        if sy < -screen_height / 2 { sy += world_manager.world_map.map.height; }
        if sy >= world_manager.world_map.map.height - screen_height / 2 { sy -= world_manager.world_map.map.height; }
        
        if sx >= 0 && sx < screen_width && sy >= 0 && sy < screen_height {
            if visible_tiles.contains(&(pos.x, pos.y)) || (id.is_some() && id.unwrap().name == "Hero") {
                ctx.set(sx, sy, render.fg, render.bg, render.glyph);
            }
        }
    }

    // HUD
    ctx.draw_hollow_box(0, 40, 79, 9, RGB::named(WHITE), RGB::named(BLACK));
    let mut stats_data = (100, 100);
    for (_entity, (stats, id)) in world_manager.world.query::<(&BaseStats, &Identity)>().iter() {
        if id.name == "Hero" {
            stats_data = (stats.hp, stats.max_hp);
        }
    }
    ctx.print(2, 42, &format!("HUD: HP: {}/{} | Pos: ({},{})", 
        stats_data.0, stats_data.1, player_pos.x, player_pos.y));
    ctx.print(2, 44, "[Arrows/Vi/Numpad] Move | [M] Main Menu");
}
