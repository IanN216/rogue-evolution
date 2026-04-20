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
use crate::utils::ui_constants::*;

pub fn tick(ctx: &mut BTerm, wm: &mut WorldManager, selection: usize) -> Option<RunState> {
    let mut draw_batch = DrawBatch::new();
    
    // Limpieza de Consolas
    draw_batch.target(0);
    draw_batch.cls();
    draw_batch.target(1);
    draw_batch.cls();
    draw_batch.target(2);
    draw_batch.cls();

    let center_x = LOGICAL_WIDTH / 2;
    let center_y = LOGICAL_HEIGHT / 3; // Subimos el anclaje al tercio superior

    draw_batch.target(2); // UI Layer
    let bw = 40;
    let bh = 12;
    
    // Offset centrado basado en constantes lógicas
    let x = center_x - (bw / 2);
    let y = center_y - (bh / 2);
    
    draw_batch.draw_hollow_box(
        Rect::with_size(x, y, bw, bh),
        ColorPair::new(WHITE, BLACK)
    );
    
    let title = "SISTEMA EN PAUSA";
    draw_batch.print_color(
        Point::new(x + (bw / 2) - (title.len() as i32 / 2), y - 2),
        title,
        ColorPair::new(YELLOW, BLACK)
    );

    if selection < 3 {
        let items = ["Continuar", "Guardar Partida", "Salir"];
        for (i, item) in items.iter().enumerate() {
            let color = if i == selection { YELLOW } else { WHITE };
            let marker = if i == selection { "-> " } else { "   " };
            let text = format!("{}{}", marker, item);
            draw_batch.print_color(
                Point::new(x + (bw / 2) - (text.len() as i32 / 2), y + 3 + i as i32 * 2),
                text,
                ColorPair::new(color, BLACK)
            );
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => {
                    let next = if selection > 0 { selection - 1 } else { 2 };
                    draw_batch.submit(0).expect("Batch submission failed");
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Down => {
                    let next = if selection < 2 { selection + 1 } else { 0 };
                    draw_batch.submit(0).expect("Batch submission failed");
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Return => {
                    match selection {
                        0 => {
                            draw_batch.submit(0).expect("Batch submission failed");
                            return Some(RunState::InGame);
                        }
                        1 => {
                            save_current_world(wm);
                            draw_batch.submit(0).expect("Batch submission failed");
                            return Some(RunState::PauseMenu { selection: 0 });
                        }
                        2 => {
                            draw_batch.submit(0).expect("Batch submission failed");
                            return Some(RunState::PauseMenu { selection: 3 });
                        }
                        _ => {}
                    }
                }
                VirtualKeyCode::Escape => {
                    draw_batch.submit(0).expect("Batch submission failed");
                    return Some(RunState::InGame);
                }
                _ => {}
            }
        }
    } else {
        let msg = "¿SALIR DEL ENTORNO?";
        draw_batch.print_color(
            Point::new(x + (bw / 2) - (msg.len() as i32 / 2), y + 2),
            msg,
            ColorPair::new(RED, BLACK)
        );
        let items = ["Menú de Inicio", "Cerrar Juego"];
        let sub_idx = selection - 3;

        for (i, item) in items.iter().enumerate() {
            let color = if i == sub_idx { YELLOW } else { WHITE };
            let marker = if i == sub_idx { "-> " } else { "   " };
            let text = format!("{}{}", marker, item);
            draw_batch.print_color(
                Point::new(x + (bw / 2) - (text.len() as i32 / 2), y + 5 + i as i32 * 2),
                text,
                ColorPair::new(color, BLACK)
            );
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::Down => {
                    let next = if selection == 3 { 4 } else { 3 };
                    draw_batch.submit(0).expect("Batch submission failed");
                    return Some(RunState::PauseMenu { selection: next });
                }
                VirtualKeyCode::Return => {
                    if selection == 3 {
                        wm.clear(LOGICAL_WIDTH, LOGICAL_HEIGHT);
                        draw_batch.submit(0).expect("Batch submission failed");
                        return Some(RunState::MainMenu { selection: super::MainMenuSelection::NewGame });
                    } else {
                        draw_batch.submit(0).expect("Batch submission failed");
                        return Some(RunState::Quit);
                    }
                }
                VirtualKeyCode::Escape => {
                    draw_batch.submit(0).expect("Batch submission failed");
                    return Some(RunState::PauseMenu { selection: 2 });
                }
                _ => {}
            }
        }
    }

    draw_batch.submit(0).expect("Batch submission failed");
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
