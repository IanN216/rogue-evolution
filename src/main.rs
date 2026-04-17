mod states;
mod core;
mod components;
mod systems;
mod utils;

use bracket_lib::prelude::*;
use states::{RunState, main_menu};

struct State {
    pub run_state: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let new_runstate = match self.run_state {
            RunState::MainMenu { .. } => states::main_menu::tick(ctx),
            RunState::CharacterCreation => {
                ctx.set_active_console(1);
                ctx.cls();
                ctx.print_centered(25, "Character Creation - Not yet implemented");
                ctx.print_centered(27, "Press [M] to return to Main Menu");
                if let Some(VirtualKeyCode::M) = ctx.key {
                    Some(RunState::MainMenu { selection: main_menu::MainMenuSelection::NewGame })
                } else {
                    None
                }
            }
            RunState::MapGen => states::map_gen_screen::tick(ctx),
            RunState::InGame => states::ingame::tick(ctx),
            RunState::Laboratory => states::laboratory::tick(ctx),
        };

        if let Some(new_state) = new_runstate {
            self.run_state = new_state;
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rogue-Evolution")
        .with_dimensions(80, 50)
        .with_tile_dimensions(8, 16)
        .with_font("vga8x16.png", 8, 16)
        .with_simple_console(80, 50, "vga8x16.png") // Layer 0: Map
        .with_simple_console_no_bg(80, 50, "vga8x16.png") // Layer 1: HUD
        .build()?;

    let gs = State {
        run_state: RunState::MainMenu { selection: main_menu::MainMenuSelection::NewGame },
    };
    main_loop(context, gs)
}
