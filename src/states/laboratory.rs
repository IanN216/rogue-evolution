use bracket_lib::prelude::*;
use super::RunState;

pub fn tick(ctx: &mut BTerm) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_centered(25, "Laboratory State - Experimental biological engineering in progress...");
    ctx.print_centered(27, "Press [M] to return to Main Menu");

    match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::M => Some(RunState::MainMenu { selection: super::main_menu::MainMenuSelection::NewGame }),
            _ => None,
        },
    }
}
