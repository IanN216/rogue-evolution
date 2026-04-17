use bracket_lib::prelude::*;
use super::RunState;

use crate::core::world::WorldManager;
use crate::core::chronometry::TimeState;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, _time_state: &mut TimeState) -> Option<RunState> {
    ctx.set_active_console(0);
    ctx.cls();
    
    // Renderizar entidades (simplificado para el demo del spec 10)
    use crate::components::stats::{Position, Renderable};
    for (_entity, (pos, render)) in world_manager.world.query::<(&Position, &Renderable)>().iter() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }

    ctx.set_active_console(1);
    ctx.cls();
    ctx.draw_hollow_box(0, 40, 79, 9, RGB::named(WHITE), RGB::named(BLACK));
    
    // Obtener datos del jugador para el HUD
    use crate::components::stats::BaseStats;
    use crate::components::identity::Identity;
    let mut player_stats = (100, 100, 0, 0); // hp, max_hp, x, y
    for (_entity, (stats, pos, id)) in world_manager.world.query::<(&BaseStats, &Position, &Identity)>().iter() {
        if id.name == "Hero" {
            player_stats = (stats.hp, stats.max_hp, pos.x, pos.y);
        }
    }

    ctx.print(2, 42, &format!("HUD: HP: {}/{} | Level: 1 | Pos: ({},{})", 
        player_stats.0, player_stats.1, player_stats.2, player_stats.3));
    ctx.print(2, 44, "Press [M] to return to Main Menu");

    match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::M => Some(RunState::MainMenu { selection: super::main_menu::MainMenuSelection::NewGame }),
            _ => None,
        },
    }
}
