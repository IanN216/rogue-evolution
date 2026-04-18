use bracket_lib::prelude::*;
use super::{RunState, MainMenuSelection};
use crate::core::world::WorldManager;
use std::fs;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager, selection: MainMenuSelection) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();

    ctx.print_color_centered(5, RGB::named(YELLOW), RGB::named(BLACK), "ROGUE-EVOLUTION");
    ctx.print_color_centered(7, RGB::named(CYAN), RGB::named(BLACK), "Celeron N2806 Optimized Edition");

    let saves = get_save_list();
    
    match selection {
        MainMenuSelection::NewGame => {
            draw_menu_item(ctx, 15, "New Game", true);
            draw_menu_item(ctx, 16, "Load Game", false);
            draw_menu_item(ctx, 17, "Laboratory", false);
            draw_menu_item(ctx, 18, "Quit", false);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Down => return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: 0 } }),
                    VirtualKeyCode::Return => {
                        world_manager.clear();
                        return Some(RunState::MapGen);
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::LoadGame { selection: idx } => {
            draw_menu_item(ctx, 15, "New Game", false);
            draw_menu_item(ctx, 16, "Load Game", true);
            draw_menu_item(ctx, 17, "Laboratory", false);
            draw_menu_item(ctx, 18, "Quit", false);

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
                            return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: idx - 1 } });
                        } else {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::NewGame });
                        }
                    }
                    VirtualKeyCode::Down => {
                        if !saves.is_empty() && idx < saves.len() - 1 {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: idx + 1 } });
                        } else {
                            return Some(RunState::MainMenu { selection: MainMenuSelection::Laboratory });
                        }
                    }
                    VirtualKeyCode::D => {
                        if !saves.is_empty() {
                            let path = format!("saves/{}", saves[idx]);
                            let _ = fs::remove_file(path);
                            return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: 0 } });
                        }
                    }
                    VirtualKeyCode::Return => {
                        if !saves.is_empty() {
                            // Logic to load specific save would go here
                            // For now, transition to InGame
                            return Some(RunState::PlayerTurn);
                        }
                    }
                    _ => {}
                }
            }
        }
        MainMenuSelection::Laboratory => {
            draw_menu_item(ctx, 15, "New Game", false);
            draw_menu_item(ctx, 16, "Load Game", false);
            draw_menu_item(ctx, 17, "Laboratory", true);
            draw_menu_item(ctx, 18, "Quit", false);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => return Some(RunState::MainMenu { selection: MainMenuSelection::LoadGame { selection: 0 } }),
                    VirtualKeyCode::Down => return Some(RunState::MainMenu { selection: MainMenuSelection::Quit }),
                    VirtualKeyCode::Return => return Some(RunState::Laboratory),
                    _ => {}
                }
            }
        }
        MainMenuSelection::Quit => {
            draw_menu_item(ctx, 15, "New Game", false);
            draw_menu_item(ctx, 16, "Load Game", false);
            draw_menu_item(ctx, 17, "Laboratory", false);
            draw_menu_item(ctx, 18, "Quit", true);

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Up => return Some(RunState::MainMenu { selection: MainMenuSelection::Laboratory }),
                    VirtualKeyCode::Return => std::process::exit(0),
                    _ => {}
                }
            }
        }
    }

    None
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
