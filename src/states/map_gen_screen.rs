use bracket_lib::prelude::*;
use super::RunState;

use crate::core::world::WorldManager;

pub fn tick(ctx: &mut BTerm, _world_manager: &mut WorldManager) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_centered(25, "Generating Procedural World... Please wait.");
    ctx.print_centered(27, "Press [G] to Start Playing");

    match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::G => {
                // Inicializar mundo real con Drunkard's Walk (Spec #11)
                use crate::core::map_gen::drunkard_walk;
                let map = drunkard_walk(80, 40, 12345); // Altura 40 para dejar espacio al HUD
                
                use crate::components::stats::{Position, BaseStats, Renderable, Viewshed};
                use crate::components::identity::Identity;
                use crate::components::progression::Experience;

                // Guardar mapa en WorldMap (simplificado para el demo)
                // En una implementación final, esto iría a través de RegionData
                
                _world_manager.world.spawn((
                    Position { x: 40, y: 20 },
                    Renderable { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK) },
                    BaseStats { hp: 100, max_hp: 100, attack: 10, defense: 10 },
                    Viewshed { visible_tiles: Vec::new(), range: 20, dirty: true },
                    Identity { name: "Hero".to_string(), title: None, kingdom_id: 0 },
                    Experience::new(),
                ));

                Some(RunState::AwaitingInput)
            }
            _ => None,
        },
    }
}
