use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::core::map_gen::drunkard_walk;
use crate::components::stats::{Position, Renderable, Viewshed, BaseStats};
use crate::components::identity::Identity;
use crate::components::progression::Experience;

pub fn tick(ctx: &mut BTerm, wm: &mut WorldManager) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_centered(25, "Generando Mundo Procedural... Por favor espere.");
    ctx.print_centered(27, "Presione [G] para Iniciar la Aventura");

    if let Some(VirtualKeyCode::G) = ctx.key {
        // 1. Generar el mapa inicial (Spec-11)
        wm.world_map.map = drunkard_walk(80, 50, 8888); // Semilla fija para pruebas
        
        // 2. Spawnear al Jugador en una posición válida (Suelo)
        let player_start = Position { x: 40, y: 25 }; 
        wm.world.spawn((
            player_start,
            Renderable { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK) },
            Viewshed { visible_tiles: Vec::new(), range: 12, dirty: true },
            Identity { name: "Hero".to_string(), title: None, kingdom_id: 0 },
            BaseStats { hp: 100, max_hp: 100, attack: 10, defense: 5 },
            Experience::new(),
        ));

        return Some(RunState::AwaitingInput);
    }
    None
}
