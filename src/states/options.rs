use bracket_lib::prelude::*;
use super::{RunState, MainMenuSelection};
use crate::utils::config::{Settings, DisplayMode};
use crate::utils::ui_constants::*;

pub fn tick(ctx: &mut BTerm, selection: usize) -> Option<RunState> {
    let mut draw_batch = DrawBatch::new();
    
    // Limpieza de Consolas
    draw_batch.target(0);
    draw_batch.cls();
    draw_batch.target(1);
    draw_batch.cls();
    draw_batch.target(2);
    draw_batch.cls();

    let center_x = LOGICAL_WIDTH / 2;
    let center_y = LOGICAL_HEIGHT / 2;

    draw_batch.target(2); // UI Layer
    let title = "CONFIGURACIÓN";
    draw_batch.print_color(
        Point::new(center_x - (title.len() as i32 / 2), center_y - 10),
        title,
        ColorPair::new(YELLOW, BLACK)
    );

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
        let color = if i == selection { YELLOW } else { WHITE };
        let marker = if i == selection { ">> " } else { "   " };
        let text = format!("{}{}", marker, option);
        draw_batch.print_color(
            Point::new(center_x - (text.len() as i32 / 2), center_y + i as i32),
            text,
            ColorPair::new(color, BLACK)
        );
    }

    let footer = "Nota: Los cambios de resolución requieren reiniciar el programa.";
    draw_batch.print_color(
        Point::new(center_x - (footer.len() as i32 / 2), center_y + 10),
        footer,
        ColorPair::new(GRAY, BLACK)
    );

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => {
                let new_selection = if selection > 0 { selection - 1 } else { options.len() - 1 };
                draw_batch.submit(0).expect("Batch submission failed");
                return Some(RunState::Options { selection: new_selection });
            }
            VirtualKeyCode::Down => {
                let new_selection = if selection < options.len() - 1 { selection + 1 } else { 0 };
                draw_batch.submit(0).expect("Batch submission failed");
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
                        draw_batch.submit(0).expect("Batch submission failed");
                        return Some(RunState::MainMenu { selection: MainMenuSelection::Options });
                    }
                    _ => {}
                }
            }
            VirtualKeyCode::Escape => {
                draw_batch.submit(0).expect("Batch submission failed");
                return Some(RunState::MainMenu { selection: MainMenuSelection::Options });
            }
            _ => {}
        }
    }

    draw_batch.submit(0).expect("Batch submission failed");
    None
}
