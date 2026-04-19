use bracket_lib::prelude::*;
use super::{RunState, MainMenuSelection};
use crate::core::world::WorldManager;
use crate::utils::persistence::load_region;
use crate::components::stats::{Renderable, BaseStats};
use crate::components::identity::Identity;
use crate::components::progression::Humanoid;
use crate::components::items::{Blighted, InfectionSource};
use std::fs;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, selection: MainMenuSelection) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();

    ctx.print_color_centered(5, RGB::named(YELLOW), RGB::named(BLACK), "ROGUE-EVOLUTION");
    ctx.print_color_centered(7, RGB::named(CYAN), RGB::named(BLACK), "Celeron N2806 Optimized Edition");

    match selection {
        MainMenuSelection::NewGame => {
            draw_main_menu_items(ctx, 0);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Down => {
                        let saves = get_save_list();
                        return Some(RunState::MainMenu { 
                            selection: MainMenuSelection::LoadGame { 
                                selection: 0, 
                                cached_saves: saves 
                            } 
                        });
                    }
                    VirtualKeyCode::Return => {
                        world_manager.clear();
                        return Some(RunState::MapGen);
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::LoadGame { selection: idx, cached_saves: saves } => {
            draw_main_menu_items(ctx, 1);

            // Draw save list
            let mut y = 22;
            ctx.print_color_centered(20, RGB::named(GRAY), RGB::named(BLACK), "--- SELECT SAVE ---");
            if saves.is_empty() {
                ctx.print_color_centered(y, RGB::named(RED), RGB::named(BLACK), "No saves found");
            } else {
                for (i, save) in saves.iter().enumerate() {
                    let color = if i == idx { RGB::named(YELLOW) } else { RGB::named(WHITE) };
                    let marker = if i == idx { "-> " } else { "   " };
                    ctx.print_color_centered(y, color, RGB::named(BLACK), format!("{}{}", marker, save));
                    y += 1;
                }
                ctx.print_color_centered(35, RGB::named(GRAY), RGB::named(BLACK), "[D] Delete Selected | [Enter] Load");
            }

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => {
                        if idx > 0 {
                            return Some(RunState::MainMenu { 
                                selection: MainMenuSelection::LoadGame { selection: idx - 1, cached_saves: saves } 
                            });
                        } else {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::NewGame });
                        }
                    }
                    VirtualKeyCode::Down => {
                        if !saves.is_empty() && idx < saves.len() - 1 {
                            return Some(RunState::MainMenu { 
                                selection: MainMenuSelection::LoadGame { selection: idx + 1, cached_saves: saves } 
                            });
                        } else {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::Laboratory });
                        }
                    }
                    VirtualKeyCode::D => {
                        if !saves.is_empty() {
                            return Some(RunState::MainMenu { 
                                selection: MainMenuSelection::ConfirmDelete { selection: idx, cached_saves: saves } 
                            });
                        }
                    }
                    VirtualKeyCode::Return => {
                        if !saves.is_empty() {
                            let save_name = &saves[idx];
                            if let Some((x, y)) = parse_save_name(save_name) {
                                match load_region(x, y) {
                                    Ok(region) => {
                                        world_manager.clear();
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
                                        return Some(RunState::InGame);
                                    }
                                    Err(e) => {
                                        ctx.print_color_centered(40, RGB::named(RED), RGB::named(BLACK), format!("Error loading save: {}", e));
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
            ctx.print_color_centered(20, RGB::named(RED), RGB::named(BLACK), "ARE YOU SURE YOU WANT TO DELETE THIS SAVE?");
            ctx.print_color_centered(22, RGB::named(WHITE), RGB::named(BLACK), format!("File: {}", saves[idx]));
            ctx.print_color_centered(24, RGB::named(YELLOW), RGB::named(BLACK), "[Y] Confirm Delete | [N] Cancel");

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Y => {
                        let path = format!("saves/{}", saves[idx]);
                        let _ = fs::remove_file(path);
                        let new_saves = get_save_list();
                        return Some(RunState::MainMenu { 
                            selection: MainMenuSelection::LoadGame { selection: 0, cached_saves: new_saves } 
                        });
                    }
                    VirtualKeyCode::N | VirtualKeyCode::Escape => {
                        return Some(RunState::MainMenu { 
                            selection: MainMenuSelection::LoadGame { selection: idx, cached_saves: saves } 
                        });
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::Laboratory => {
            draw_main_menu_items(ctx, 2);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => {
                        let saves = get_save_list();
                        return Some(RunState::MainMenu { 
                            selection: MainMenuSelection::LoadGame { selection: 0, cached_saves: saves } 
                        });
                    }
                    VirtualKeyCode::Down => return Some(RunState::MainMenu { selection: MainMenuSelection::Options }),
                    VirtualKeyCode::Return => return Some(RunState::Laboratory),
                    _ => {}
                }
            }
        }
        MainMenuSelection::Options => {
            draw_main_menu_items(ctx, 3);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => return Some(RunState::MainMenu { selection: MainMenuSelection::Laboratory }),
                    VirtualKeyCode::Down => return Some(RunState::MainMenu { selection: MainMenuSelection::Quit }),
                    VirtualKeyCode::Return => return Some(RunState::Options { selection: 0 }),
                    _ => {}
                }
            }
        }
        MainMenuSelection::Quit => {
            draw_main_menu_items(ctx, 4);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => return Some(RunState::MainMenu { selection: MainMenuSelection::Options }),
                    VirtualKeyCode::Return => std::process::exit(0),
                    _ => {}
                }
            }
        }
    }

    None
}

fn draw_main_menu_items(ctx: &mut BTerm, selected_idx: usize) {
    let items = ["New Game", "Load Game", "Laboratory", "Options", "Quit"];
    for (i, item) in items.iter().enumerate() {
        draw_menu_item(ctx, 15 + i as i32, item, i == selected_idx);
    }
}

fn draw_menu_item(ctx: &mut BTerm, y: i32, text: &str, selected: bool) {
    let color = if selected { RGB::named(YELLOW) } else { RGB::named(WHITE) };
    let marker = if selected { ">> " } else { "   " };
    ctx.print_color_centered(y, color, RGB::named(BLACK), format!("{}{}", marker, text));
}

fn get_save_list() -> Vec<String> {
    let mut saves = Vec::new();
    if let Ok(entries) = fs::read_dir("saves") {
        for entry in entries.flatten() {
            if let Some(s) = entry.file_name().to_str() {
                if s.ends_with(".bin") {
                    saves.push(s.to_string());
                }
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
