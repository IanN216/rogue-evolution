use bracket_lib::prelude::*;
use super::RunState;

pub fn tick(ctx: &mut BTerm) -> Option<RunState> {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_centered(25, "Generating Procedural World... Please wait.");
    ctx.print_centered(27, "Press [G] to Start Playing");

    match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::G => Some(RunState::InGame),
            _ => None,
        },
    }
}
