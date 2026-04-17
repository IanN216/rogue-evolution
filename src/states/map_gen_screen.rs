use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_centered(25, "Generating Procedural World... Please wait.");
    ctx.print_centered(27, "Press [G] to Start Playing");

    match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::G => {
                // 1. Generar Mapa Real con Drunkard's Walk
                use crate::core::map_gen::drunkard_walk;
                let map = drunkard_walk(80, 50, 12345);
                
                // 2. Inyectar mapa en WorldManager
                world_manager.world_map.map = map;

                // 3. Spawn del Jugador en una posición válida (Floor)
                use crate::components::stats::{Position, BaseStats, Renderable, Viewshed};
                use crate::components::identity::Identity;
                use crate::components::progression::Experience;
                use crate::core::map::TileType;

                let mut player_x = 40;
                let mut player_y = 25;
                
                // Buscar el primer suelo disponible cerca del centro
                'outer: for y in 25..50 {
                    for x in 40..80 {
                        if world_manager.world_map.map.tiles[world_manager.world_map.map.xy_idx(x, y)] == TileType::Floor {
                            player_x = x;
                            player_y = y;
                            break 'outer;
                        }
                    }
                }

                let player = world_manager.world.spawn((
                    Position { x: player_x, y: player_y },
                    Renderable { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK) },
                    BaseStats { hp: 100, max_hp: 100, attack: 10, defense: 10 },
                    Viewshed { visible_tiles: Vec::new(), range: 12, dirty: true },
                    Identity { name: "Hero".to_string(), title: None, kingdom_id: 0 },
                    Experience::new(),
                ));

                // 4. Spawn de algunos monstruos de prueba para verificar IA de hordas
                use crate::components::kingdom::{HordeLeader, HordeMember};
                use crate::components::ai::HordeTarget;

                let leader = world_manager.world.spawn((
                    Position { x: player_x - 5, y: player_y - 5 },
                    Renderable { glyph: to_cp437('L'), fg: RGB::named(RED), bg: RGB::named(BLACK) },
                    BaseStats { hp: 30, max_hp: 30, attack: 5, defense: 2 },
                    Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true },
                    HordeLeader,
                    HordeTarget { target_entity: player },
                    Identity { name: "Orc Leader".to_string(), title: None, kingdom_id: 1 },
                ));

                for i in 0..5 {
                    world_manager.world.spawn((
                        Position { x: player_x - 6 - i, y: player_y - 5 },
                        Renderable { glyph: to_cp437('o'), fg: RGB::named(ORANGE), bg: RGB::named(BLACK) },
                        HordeMember { leader_entity: leader },
                        BaseStats { hp: 10, max_hp: 10, attack: 2, defense: 1 },
                    ));
                }

                Some(RunState::AwaitingInput)
            }
            _ => None,
        },
    }
}
