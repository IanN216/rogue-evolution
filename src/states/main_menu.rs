use bracket_lib::prelude::*;
use super::{RunState, MainMenuSelection};
use crate::core::world::WorldManager;
use crate::utils::persistence::load_region;
use crate::components::stats::{Renderable, BaseStats};
use crate::components::identity::Identity;
use crate::components::progression::Humanoid;
use crate::components::items::{Blighted, InfectionSource};
use crate::utils::ui_constants::*;
use std::fs;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, selection: MainMenuSelection) -> Option<RunState> {
    let mut draw_batch = DrawBatch::new();
    
    // Limpieza de Consolas mediante Batch
    for i in 0..3 {
        draw_batch.target(i);
        draw_batch.cls();
    }

    let center_y = LOGICAL_HEIGHT / 2;

    draw_batch.target(2); // Capa de UI
    draw_batch.print_color_centered(center_y - 12, "ROGUE-EVOLUTION", ColorPair::new(YELLOW, BLACK));
    draw_batch.print_color_centered(center_y - 10, "Celeron N2806 Optimized Edition", ColorPair::new(CYAN, BLACK));

    match selection {
        MainMenuSelection::NewGame => {
            draw_main_menu_items(&mut draw_batch, 0, center_y);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Down => {
                        let saves = get_save_list();
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { 
                            selection: MainMenuSelection::LoadGame { selection: 0, cached_saves: saves } 
                        });
                    }
                    VirtualKeyCode::Return => {
                        world_manager.clear(LOGICAL_WIDTH, LOGICAL_HEIGHT);
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::CharacterCreation { selection: 0 });
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::LoadGame { selection: idx, cached_saves: saves } => {
            draw_main_menu_items(&mut draw_batch, 1, center_y);

            let start_y = center_y + 2;
            draw_batch.print_color_centered(start_y, "--- SELECT SAVE ---", ColorPair::new(GRAY, BLACK));
            
            if saves.is_empty() {
                draw_batch.print_color_centered(start_y + 2, "No saves found", ColorPair::new(RED, BLACK));
            } else {
                for (i, save) in saves.iter().enumerate() {
                    if i > 10 { break; } 
                    let color = if i == idx { YELLOW } else { WHITE };
                    let marker = if i == idx { "-> " } else { "   " };
                    let text = format!("{}{}", marker, save);
                    draw_batch.print_color_centered(start_y + 2 + i as i32, text, ColorPair::new(color, BLACK));
                }
                draw_batch.print_color_centered(start_y + 12, "[D] Delete Selected | [Enter] Load", ColorPair::new(GRAY, BLACK));
            }

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => {
                        draw_batch.submit(1000).expect("Batch failed");
                        if idx > 0 {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: idx - 1, cached_saves: saves } });
                        } else {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::NewGame });
                        }
                    }
                    VirtualKeyCode::Down => {
                        draw_batch.submit(1000).expect("Batch failed");
                        if !saves.is_empty() && idx < saves.len() - 1 {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: idx + 1, cached_saves: saves } });
                        } else {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::Laboratory });
                        }
                    }
                    VirtualKeyCode::D => {
                        if !saves.is_empty() {
                            draw_batch.submit(1000).expect("Batch failed");
                            return Some(RunState::MainMenu { selection: MainMenuSelection::ConfirmDelete { selection: idx, cached_saves: saves } });
                        }
                    }
                    VirtualKeyCode::Return => {
                        if !saves.is_empty() && idx < saves.len() {
                            let save_name = &saves[idx];
                            if let Some((x, y)) = parse_save_name(save_name) {
                                match load_region(x, y) {
                                    Ok(region) => {
                                        world_manager.clear(LOGICAL_WIDTH, LOGICAL_HEIGHT);
                                        world_manager.world_map.map.tiles = region.tiles.clone();
                                        world_manager.world_map.map.update_map_metadata(None);

                                        for snp in region.entities {
                                            let e = world_manager.world.spawn((
                                                snp.position,
                                                snp.renderable.unwrap_or(Renderable { glyph: to_cp437('?'), fg: RGB::named(RED), bg: RGB::named(BLACK) }),
                                                snp.base_stats.unwrap_or(BaseStats { hp: 1, max_hp: 1, attack: 0, defense: 0 }),
                                                snp.identity.unwrap_or(Identity { name: "Loaded".to_string(), title: None, kingdom_id: 0 }),
                                            ));

                                            if let Some(c) = snp.viewshed { world_manager.world.insert_one(e, c).unwrap(); }
                                            if let Some(c) = snp.genetics { world_manager.world.insert_one(e, c).unwrap(); }
                                            if let Some(c) = snp.kingdom_member { world_manager.world.insert_one(e, c).unwrap(); }
                                            if let Some(c) = snp.metabolism { world_manager.world.insert_one(e, c).unwrap(); }
                                            if let Some(c) = snp.experience { world_manager.world.insert_one(e, c).unwrap(); }
                                            if let Some(c) = snp.abilities { world_manager.world.insert_one(e, c).unwrap(); }
                                            if snp.is_humanoid { world_manager.world.insert_one(e, Humanoid).unwrap(); }
                                            if let Some(c) = snp.item { world_manager.world.insert_one(e, c).unwrap(); }
                                            if let Some(c) = snp.weapon { world_manager.world.insert_one(e, c).unwrap(); }
                                            if snp.is_blighted { world_manager.world.insert_one(e, Blighted).unwrap(); }
                                            if snp.is_infection_source { world_manager.world.insert_one(e, InfectionSource).unwrap(); }
                                        }
                                        world_manager.world_map.loaded_regions.insert((x, y));
                                        draw_batch.submit(1000).expect("Batch failed");
                                        return Some(RunState::InGame);
                                    }
                                    Err(e) => {
                                        let err_msg = format!("Error: {}", e);
                                        draw_batch.print_color_centered(start_y + 14, err_msg, ColorPair::new(RED, BLACK));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::ConfirmDelete { selection: idx, cached_saves: saves } => {
            let start_y = center_y + 2;
            draw_batch.print_color_centered(start_y, "¿ELIMINAR REGISTRO?", ColorPair::new(RED, BLACK));
            draw_batch.print_color_centered(start_y + 2, &saves[idx], ColorPair::new(WHITE, BLACK));
            draw_batch.print_color_centered(start_y + 4, "[Y] Confirmar | [N] Cancelar", ColorPair::new(YELLOW, BLACK));

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Y => {
                        let path = format!("saves/{}", saves[idx]);
                        let _ = fs::remove_file(path);
                        let new_saves = get_save_list();
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { 
                            selection: MainMenuSelection::LoadGame { selection: 0, cached_saves: new_saves } 
                        });
                    }
                    VirtualKeyCode::N | VirtualKeyCode::Escape => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: idx, cached_saves: saves } });
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::Laboratory => {
            draw_main_menu_items(&mut draw_batch, 2, center_y);
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Return => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::Laboratory);
                    }
                    VirtualKeyCode::Up => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: 0, cached_saves: get_save_list() } });
                    }
                    VirtualKeyCode::Down => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::Options });
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::Options => {
            draw_main_menu_items(&mut draw_batch, 3, center_y);
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Return => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::Options { selection: 0 });
                    }
                    VirtualKeyCode::Up => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::Laboratory });
                    }
                    VirtualKeyCode::Down => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::Quit });
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::Quit => {
            draw_main_menu_items(&mut draw_batch, 4, center_y);
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Return => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::Quit);
                    }
                    VirtualKeyCode::Up => {
                        draw_batch.submit(1000).expect("Batch failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::Options });
                    }
                    _ => {}
                }
            }
        }
    }

    draw_batch.submit(1000).expect("Batch submission failed");
    None
}

fn draw_main_menu_items(draw_batch: &mut DrawBatch, selected_idx: usize, center_y: i32) {
    let items = ["Nueva Partida", "Cargar Mundo", "Laboratorio", "Opciones", "Salir"];
    for (i, item) in items.iter().enumerate() {
        let color = if i == selected_idx { YELLOW } else { WHITE };
        let marker = if i == selected_idx { ">> " } else { "   " };
        let text = format!("{}{}", marker, item);
        draw_batch.print_color_centered((center_y - 6) + i as i32 * 2, text, ColorPair::new(color, BLACK));
    }
}

fn get_save_list() -> Vec<String> {
    let mut saves = Vec::new();
    if let Ok(entries) = fs::read_dir("saves") {
        for entry in entries.flatten() {
            if let Some(s) = entry.file_name().to_str() {
                if s.ends_with(".bin") { saves.push(s.to_string()); }
            }
        }
    }
    saves.sort();
    saves
}

fn parse_save_name(name: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = name.strip_suffix(".bin")?.split('_').collect();
    if parts.len() == 3 && parts[0] == "region" {
        let x = parts[1].parse().ok()?;
        let y = parts[2].parse().ok()?;
        Some((x, y))
    } else {
        None
    }
}
