use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;

pub fn tick(ctx: &mut BTerm, _wm: &mut WorldManager, selection: usize) -> Option<RunState> {
    // 1. Limpieza de Búfer Triple
    for i in 0..3 {
        ctx.set_active_console(i);
        ctx.cls();
    }

    let (sw, sh) = ctx.get_char_size();
    let center_x = sw as i32 / 2;
    let center_y = sh as i32 / 2;

    ctx.set_active_console(1);
    ctx.print_color(center_x - 10, center_y - 12, RGB::named(YELLOW), RGB::named(BLACK), "CREACIÓN GENÉTICA");

    let attributes = [
        "Metabolismo Acelerado",
        "Visión Térmica",
        "Resistencia al Blight",
        "Regeneración Celular",
        "Estructura Ósea Reforzada"
    ];

    let bw = 40;
    let bh = attributes.len() as i32 + 4;
    ctx.draw_hollow_box(center_x - (bw/2), center_y - (bh/2), bw, bh, RGB::named(WHITE), RGB::named(BLACK));

    for (i, attr) in attributes.iter().enumerate() {
        let color = if i == selection { RGB::named(CYAN) } else { RGB::named(WHITE) };
        let marker = if i == selection { "> " } else { "  " };
        let text = format!("{}{}", marker, attr);
        ctx.print_color(center_x - (text.len() as i32 / 2), center_y - (attributes.len() as i32 / 2) + i as i32, color, RGB::named(BLACK), text);
    }

    ctx.print_color(center_x - 15, center_y + bh, RGB::named(GRAY), RGB::named(BLACK), "[Flechas] Navegar | [Enter] Confirmar");

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => {
                let next = if selection > 0 { selection - 1 } else { attributes.len() - 1 };
                return Some(RunState::CharacterCreation { selection: next });
            }
            VirtualKeyCode::Down => {
                let next = if selection < attributes.len() - 1 { selection + 1 } else { 0 };
                return Some(RunState::CharacterCreation { selection: next });
            }
            VirtualKeyCode::Return => {
                return Some(RunState::MapGen { phase: 0, progress: 0.0, phase_step: 0 });
            }
            VirtualKeyCode::Escape => {
                return Some(RunState::MainMenu { selection: super::MainMenuSelection::NewGame });
            }
            _ => {}
        }
    }

    None
}
