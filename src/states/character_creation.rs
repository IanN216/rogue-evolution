use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::utils::ui_constants::*;

pub fn tick(ctx: &mut BTerm, _wm: &mut WorldManager, selection: usize) -> Option<RunState> {
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
    let title = "CREACIÓN GENÉTICA";
    draw_batch.print_color(
        Point::new(center_x - (title.len() as i32 / 2), center_y - 12),
        title,
        ColorPair::new(YELLOW, BLACK)
    );

    let attributes = [
        "Metabolismo Acelerado",
        "Visión Térmica",
        "Resistencia al Blight",
        "Regeneración Celular",
        "Estructura Ósea Reforzada"
    ];

    let bw = 40;
    let bh = attributes.len() as i32 + 4;
    draw_batch.draw_hollow_box(
        Rect::with_size(center_x - (bw / 2), center_y - (bh / 2), bw, bh),
        ColorPair::new(WHITE, BLACK)
    );

    for (i, attr) in attributes.iter().enumerate() {
        let color = if i == selection { CYAN } else { WHITE };
        let marker = if i == selection { "> " } else { "  " };
        let text = format!("{}{}", marker, attr);
        draw_batch.print_color(
            Point::new(center_x - (text.len() as i32 / 2), center_y - (attributes.len() as i32 / 2) + i as i32),
            text,
            ColorPair::new(color, BLACK)
        );
    }

    let footer = "[Flechas] Navegar | [Enter] Confirmar";
    draw_batch.print_color(
        Point::new(center_x - (footer.len() as i32 / 2), center_y + bh),
        footer,
        ColorPair::new(GRAY, BLACK)
    );

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => {
                let next = if selection > 0 { selection - 1 } else { attributes.len() - 1 };
                draw_batch.submit(0).expect("Batch submission failed");
                return Some(RunState::CharacterCreation { selection: next });
            }
            VirtualKeyCode::Down => {
                let next = if selection < attributes.len() - 1 { selection + 1 } else { 0 };
                draw_batch.submit(0).expect("Batch submission failed");
                return Some(RunState::CharacterCreation { selection: next });
            }
            VirtualKeyCode::Return => {
                draw_batch.submit(0).expect("Batch submission failed");
                return Some(RunState::MapGen { phase: 0, progress: 0.0, phase_step: 0 });
            }
            VirtualKeyCode::Escape => {
                draw_batch.submit(0).expect("Batch submission failed");
                return Some(RunState::MainMenu { selection: super::MainMenuSelection::NewGame });
            }
            _ => {}
        }
    }

    draw_batch.submit(0).expect("Batch submission failed");
    None
}
