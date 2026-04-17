use bracket_lib::prelude::*;
use super::RunState;

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    Laboratory,
    Quit,
}

use crate::core::world::WorldManager;

pub fn tick(ctx: &mut BTerm, world_manager: &mut WorldManager) -> Option<RunState> {
    ctx.set_active_console(1); // HUD / UI Layer
    ctx.cls();

    let menu_x = 30;
    let menu_y = 20;

    ctx.print_color_centered(10, RGB::named(YELLOW), RGB::named(BLACK), "ROGUE-EVOLUTION");
    ctx.print_color_centered(12, RGB::named(CYAN), RGB::named(BLACK), "Celeron N2806 Optimized Edition");

    // This is a placeholder for the actual selection logic.
    // In a real implementation, we would pass the current selection in the state.
    // For now, let's just draw the options.

    ctx.print_color(menu_x, menu_y, RGB::named(WHITE), RGB::named(BLACK), "[N] New Game");
    ctx.print_color(menu_x, menu_y + 1, RGB::named(WHITE), RGB::named(BLACK), "[L] Laboratory");
    ctx.print_color(menu_x, menu_y + 2, RGB::named(WHITE), RGB::named(BLACK), "[Q] Quit");

    match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::N => {
                world_manager.clear();
                Some(RunState::MapGen)
            }
            VirtualKeyCode::L => Some(RunState::Laboratory),
            VirtualKeyCode::Q => {
                std::process::exit(0);
            }
            _ => None,
        },
    }
}
