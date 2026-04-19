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
    // 1. Limpieza de Búfer Triple
    for i in 0..3 {
        ctx.set_active_console(i);
        ctx.cls();
    }

    let (sw, sh) = ctx.get_char_size();
    let center_x = sw as i32 / 2;
    let center_y = sh as i32 / 2;

    // 2. Dibujo en Capa 1 (UI)
    ctx.set_active_console(1);
    let bw = 40;
    let bh = 12;
    ctx.draw_hollow_box(center_x - (bw / 2), center_y - (bh / 2), bw, bh, RGB::named(WHITE), RGB::named(BLACK));
    
    let title = "SISTEMA EN PAUSA";
    ctx.print_color(center_x - (title.len() as i32 / 2), center_y - (bh / 2) - 2, RGB::named(YELLOW), RGB::named(BLACK), title);

    if selection < 3 {
        let items = ["Continuar", "Guardar Partida", "Salir"];
        for (i, item) in items.iter().enumerate() {
            let color = if i == selection { RGB::named(YELLOW) } else { RGB::named(WHITE) };
            let marker = if i == selection { "-> " } else { "   " };
            let text = format!("{}{}", marker, item);
            ctx.print_color(center_x - (text.len() as i32 / 2), center_y - (bh / 2) + 3 + i as i32 * 2, color, RGB::named(BLACK), text);
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
                        2 => return Some(RunState::PauseMenu { selection: 3 }), 
                        _ => {}
                    }
                }
                VirtualKeyCode::Escape => return Some(RunState::InGame),
                _ => {}
            }
        }
    } else {
        let msg = "¿SALIR DEL ENTORNO?";
        ctx.print_color(center_x - (msg.len() as i32 / 2), center_y - (bh / 2) + 2, RGB::named(RED), RGB::named(BLACK), msg);
        let items = ["Menú de Inicio", "Cerrar Juego"];
        let sub_idx = selection - 3;

        for (i, item) in items.iter().enumerate() {
            let color = if i == sub_idx { RGB::named(YELLOW) } else { RGB::named(WHITE) };
            let marker = if i == sub_idx { "-> " } else { "   " };
            let text = format!("{}{}", marker, item);
            ctx.print_color(center_x - (text.len() as i32 / 2), center_y - (bh / 2) + 5 + i as i32 * 2, color, RGB::named(BLACK), text);
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::Down => {
                    let next = if selection == 3 { 4 } else { 3 };
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Return => {
                    if selection == 3 {
                        wm.clear(sw as i32, sh as i32);
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
