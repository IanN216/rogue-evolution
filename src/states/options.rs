use bracket_lib::prelude::*;
use super::{RunState, MainMenuSelection};
use crate::utils::config::{Settings, DisplayMode};

pub fn tick(ctx: &mut BTerm, selection: usize) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();

    ctx.print_color_centered(5, RGB::named(YELLOW), RGB::named(BLACK), "OPTIONS");

    let mut settings = Settings::load();

    let options = [
        format!("Windowed (80x50) {}", if settings.display_mode == DisplayMode::Windowed80x50 { "[*]" } else { "[ ]" }),
        format!("Fullscreen Native (170x48) {}", if settings.display_mode == DisplayMode::FullscreenNative170x48 { "[*]" } else { "[ ]" }),
        "Back to Main Menu".to_string(),
    ];

    for (i, option) in options.iter().enumerate() {
        let color = if i == selection { RGB::named(YELLOW) } else { RGB::named(WHITE) };
        let marker = if i == selection { ">> " } else { "   " };
        ctx.print_color_centered(15 + i as i32, color, RGB::named(BLACK), format!("{}{}", marker, option));
    }

    ctx.print_color_centered(25, RGB::named(GRAY), RGB::named(BLACK), "Note: Resolution changes may require restart.");

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => {
                let new_selection = if selection > 0 { selection - 1 } else { options.len() - 1 };
                return Some(RunState::Options { selection: new_selection });
            }
            VirtualKeyCode::Down => {
                let new_selection = if selection < options.len() - 1 { selection + 1 } else { 0 };
                return Some(RunState::Options { selection: new_selection });
            }
            VirtualKeyCode::Return => {
                match selection {
                    0 => {
                        settings.display_mode = DisplayMode::Windowed80x50;
                        settings.fullscreen = false;
                        settings.save();
                    }
                    1 => {
                        settings.display_mode = DisplayMode::FullscreenNative170x48;
                        settings.fullscreen = true;
                        settings.save();
                    }
                    2 => {
                        return Some(RunState::MainMenu { selection: MainMenuSelection::Options });
                    }
                    _ => {}
                }
            }
            VirtualKeyCode::Escape => {
                return Some(RunState::MainMenu { selection: MainMenuSelection::Options });
            }
            _ => {}
        }
    }

    None
}
