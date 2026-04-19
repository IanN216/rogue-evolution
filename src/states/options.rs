use bracket_lib::prelude::*;
use super::{RunState, MainMenuSelection};
use crate::utils::config::{Settings, DisplayMode};

pub fn tick(ctx: &mut BTerm, selection: usize) -> Option<RunState> {
    // 1. Limpieza de Búfer Triple
    for i in 0..3 {
        ctx.set_active_console(i);
        ctx.cls();
    }

    let (sw, sh) = ctx.get_char_size();
    let center_x = sw as i32 / 2;
    let center_y = sh as i32 / 2;

    ctx.set_active_console(1);
    let title = "CONFIGURACIÓN";
    ctx.print_color(center_x - (title.len() as i32 / 2), center_y - 10, RGB::named(YELLOW), RGB::named(BLACK), title);

    let mut settings = Settings::load();

    // Nombres Amigables
    let label_windowed = format!("Ventana (80x50) {}", if settings.display_mode == DisplayMode::Windowed80x50 { "[*]" } else { "[ ]" });
    let label_fullscreen = format!("Pantalla Completa (1366x768) {}", if settings.display_mode == DisplayMode::FullscreenNative170x48 { "[*]" } else { "[ ]" });

    let options = [
        label_windowed,
        label_fullscreen,
        "Guardar y Volver al Menú Principal".to_string(),
    ];

    for (i, option) in options.iter().enumerate() {
        let color = if i == selection { RGB::named(YELLOW) } else { RGB::named(WHITE) };
        let marker = if i == selection { ">> " } else { "   " };
        let text = format!("{}{}", marker, option);
        ctx.print_color(center_x - (text.len() as i32 / 2), center_y + i as i32, color, RGB::named(BLACK), text);
    }

    let footer = "Nota: Los cambios de resolución requieren reiniciar el programa.";
    ctx.print_color(center_x - (footer.len() as i32 / 2), center_y + 10, RGB::named(GRAY), RGB::named(BLACK), footer);

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
                        settings.save();
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
