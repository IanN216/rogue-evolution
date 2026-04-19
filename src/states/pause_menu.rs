use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::utils::persistence::save_region;
use crate::core::world_map::RegionData;
use crate::components::stats::{Position, Renderable, BaseStats, Viewshed, Metabolism};
use crate::components::identity::Identity;
use crate::components::kingdom::KingdomMember;
use crate::components::progression::{Experience, AbilityRegistry, Humanoid};
use crate::components::items::{Item, Weapon, Blighted, InfectionSource};
use crate::core::world_map::EntitySnapshot;

pub fn tick(ctx: &mut BTerm, wm: &mut WorldManager, selection: usize) -> Option<RunState> {
    // 1. Limpieza Total de Capas
    ctx.set_active_console(0);
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();

    let (w, h) = ctx.get_char_size();

    // Semi-transparent background simulation
    let box_width = 40;
    let box_height = 12;
    let x = (w as i32 - box_width) / 2;
    let y = (h as i32 - box_height) / 2;

    ctx.draw_hollow_box(x, y, box_width, box_height, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_color_centered(y - 2, RGB::named(YELLOW), RGB::named(BLACK), "SISTEMA EN PAUSA");

    if selection < 3 {
        // Menú Principal de Pausa
        let items = ["Continuar", "Guardar Partida", "Salir"];
        for (i, item) in items.iter().enumerate() {
            let color = if i == selection { RGB::named(YELLOW) } else { RGB::named(WHITE) };
            let marker = if i == selection { "-> " } else { "   " };
            ctx.print_color_centered(y + 3 + i as i32 * 2, color, RGB::named(BLACK), format!("{}{}", marker, item));
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => {
                    let next = if selection > 0 { selection - 1 } else { 2 };
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Down => {
                    let next = if selection < 2 { selection + 1 } else { 0 };
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Return => {
                    match selection {
                        0 => return Some(RunState::InGame),
                        1 => {
                            save_current_world(wm);
                            return Some(RunState::PauseMenu { selection: 0 });
                        }
                        2 => return Some(RunState::PauseMenu { selection: 3 }), // Sub-menú de salida
                        _ => {}
                    }
                }
                VirtualKeyCode::Escape => return Some(RunState::InGame),
                _ => {}
            }
        }
    } else {
        // Sub-menú de Salida (User Directive)
        ctx.print_color_centered(y + 2, RGB::named(RED), RGB::named(BLACK), "¿SALIR DEL ENTORNO?");
        let items = ["Menú de Inicio", "Cerrar Juego"];
        let sub_idx = selection - 3;

        for (i, item) in items.iter().enumerate() {
            let color = if i == sub_idx { RGB::named(YELLOW) } else { RGB::named(WHITE) };
            let marker = if i == sub_idx { "-> " } else { "   " };
            ctx.print_color_centered(y + 5 + i as i32 * 2, color, RGB::named(BLACK), format!("{}{}", marker, item));
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::Down => {
                    let next = if selection == 3 { 4 } else { 3 };
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Return => {
                    if selection == 3 {
                        // Limpiar mundo de la memoria al salir (User Directive)
                        wm.clear(w as i32, h as i32);
                        return Some(RunState::MainMenu { selection: super::MainMenuSelection::NewGame });
                    } else {
                        return Some(RunState::Quit);
                    }
                }
                VirtualKeyCode::Escape => return Some(RunState::PauseMenu { selection: 2 }),
                _ => {}
            }
        }
    }

    None
}

fn save_current_world(wm: &mut WorldManager) {
    let mut entities = Vec::new();
    for (entity, pos) in wm.world.query::<&Position>().iter() {
        let renderable = wm.world.get::<&Renderable>(entity).ok().map(|c| (*c).clone());
        let base_stats = wm.world.get::<&BaseStats>(entity).ok().map(|c| (*c).clone());
        let viewshed = wm.world.get::<&Viewshed>(entity).ok().map(|c| (*c).clone());
        let genetics = wm.world.get::<&crate::components::genetics::Genetics>(entity).ok().map(|c| (*c).clone());
        let identity = wm.world.get::<&Identity>(entity).ok().map(|c| (*c).clone());
        let kingdom_member = wm.world.get::<&KingdomMember>(entity).ok().map(|c| (*c).clone());
        let metabolism = wm.world.get::<&Metabolism>(entity).ok().map(|c| (*c).clone());
        let experience = wm.world.get::<&Experience>(entity).ok().map(|c| (*c).clone());
        let abilities = wm.world.get::<&AbilityRegistry>(entity).ok().map(|c| (*c).clone());
        let is_humanoid = wm.world.get::<&Humanoid>(entity).is_ok();
        let item = wm.world.get::<&Item>(entity).ok().map(|c| (*c).clone());
        let weapon = wm.world.get::<&Weapon>(entity).ok().map(|c| (*c).clone());
        let is_blighted = wm.world.get::<&Blighted>(entity).is_ok();
        let is_infection_source = wm.world.get::<&InfectionSource>(entity).is_ok();

        entities.push(EntitySnapshot {
            position: *pos,
            renderable,
            base_stats,
            viewshed,
            genetics,
            identity,
            kingdom_member,
            metabolism,
            experience,
            abilities,
            is_humanoid,
            item,
            weapon,
            is_blighted,
            is_infection_source,
        });
    }

    let data = RegionData {
        x: 0,
        y: 0,
        tiles: wm.world_map.map.tiles.clone(),
        entities,
    };

    let _ = save_region(&data);
}
