use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::core::chronometry::TimeState;
use crate::components::stats::{Position, Renderable, Viewshed, BaseStats};
use crate::components::identity::Identity;
use crate::systems::sensory::fov::process_fov;
use crate::systems::ai::swarm::process_swarm_ai;
use crate::core::map::{Map, TileType};

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, time_state: &mut TimeState, current_state: RunState) -> Option<RunState> {
    // 1. Renderizado (Siempre ocurre primero para que el jugador vea el estado actual)
    render(ctx, world_manager);

    // 2. Lógica según el estado de turno
    match current_state {
        RunState::AwaitingInput => {
            if let Some(key) = ctx.key {
                match key {
                    // Movimiento del jugador
                    VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::Numpad4 => return try_move_player(-1, 0, world_manager),
                    VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::Numpad6 => return try_move_player(1, 0, world_manager),
                    VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::Numpad8 => return try_move_player(0, -1, world_manager),
                    VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::Numpad2 => return try_move_player(0, 1, world_manager),
                    
                    // Menú
                    VirtualKeyCode::M => return Some(RunState::MainMenu { selection: super::main_menu::MainMenuSelection::NewGame }),
                    _ => {}
                }
            }
            None
        }
        RunState::PlayerTurn => {
            // El movimiento ya ocurrió en AwaitingInput, ahora procesamos consecuencias locales del jugador
            // Por ejemplo, recalcular FOV inmediatamente.
            // Para el demo, el mapa es genérico. En un juego real, pasaríamos el mapa de la región actual.
            let dummy_map = Map::new(80, 40); // Placeholder
            process_fov(&mut world_manager.world, &dummy_map, time_state);
            
            Some(RunState::MonsterTurn)
        }
        RunState::MonsterTurn => {
            time_state.update();
            
            // IA de Monstruos y Hordas (Dijkstra + Rayon)
            let dummy_map = Map::new(80, 40); // Placeholder
            process_swarm_ai(&mut world_manager.world, &dummy_map);
            
            // Otros sistemas periódicos
            // process_infection, process_metabolism, etc.
            
            Some(RunState::AwaitingInput)
        }
        _ => None
    }
}

fn try_move_player(dx: i32, dy: i32, world_manager: &mut WorldManager) -> Option<RunState> {
    let query = world_manager.world.query_mut::<(&mut Position, &mut Viewshed, &Identity)>();
    for (_entity, (pos, viewshed, id)) in query {
        if id.name == "Hero" {
            // Validar límites y muros (simplificado)
            let new_x = (pos.x + dx).clamp(0, 79);
            let new_y = (pos.y + dy).clamp(0, 39);
            
            pos.x = new_x;
            pos.y = new_y;
            viewshed.dirty = true;
            return Some(RunState::PlayerTurn);
        }
    }
    None
}

fn render(ctx: &mut BTerm, world_manager: &mut WorldManager) {
    // Capa 0: Mapa (FOV-aware)
    ctx.set_active_console(0);
    ctx.cls();
    
    // Obtener las celdas visibles del jugador
    let mut visible_tiles = std::collections::HashSet::new();
    for (_entity, (viewshed, id)) in world_manager.world.query::<(&Viewshed, &Identity)>().iter() {
        if id.name == "Hero" {
            for pt in viewshed.visible_tiles.iter() {
                visible_tiles.insert((pt.x, pt.y));
            }
        }
    }

    // Dibujar mapa base (Demo: solo puntos para suelo si es visible)
    // En implementación final, iteraríamos sobre world_map.active_regions
    for y in 0..40 {
        for x in 0..80 {
            if visible_tiles.contains(&(x, y)) {
                ctx.set(x, y, RGB::named(DARK_GREEN), RGB::named(BLACK), to_cp437('.'));
            } else {
                ctx.set(x, y, RGB::named(BLACK), RGB::named(BLACK), to_cp437(' '));
            }
        }
    }

    // Capa 1: Entidades y HUD
    ctx.set_active_console(1);
    ctx.cls();
    
    for (_entity, (pos, render, id)) in world_manager.world.query::<(&Position, &Renderable, Option<&Identity>)>().iter() {
        if visible_tiles.contains(&(pos.x, pos.y)) || (id.is_some() && id.unwrap().name == "Hero") {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }

    // HUD
    ctx.draw_hollow_box(0, 40, 79, 9, RGB::named(WHITE), RGB::named(BLACK));
    let mut player_stats = (100, 100, 0, 0);
    for (_entity, (stats, pos, id)) in world_manager.world.query::<(&BaseStats, &Position, &Identity)>().iter() {
        if id.name == "Hero" {
            player_stats = (stats.hp, stats.max_hp, pos.x, pos.y);
        }
    }
    ctx.print(2, 42, &format!("HUD: HP: {}/{} | Pos: ({},{})", 
        player_stats.0, player_stats.1, player_stats.2, player_stats.3));
    ctx.print(2, 44, "[Arrows/Vi/Numpad] Move | [M] Main Menu");
}
