mod states;
mod core;
mod components;
mod systems;
mod utils;

use bracket_lib::prelude::*;
use states::RunState;
use crate::core::world::WorldManager;
use crate::core::chronometry::TimeState;
use crate::utils::config::Settings;

pub trait FullscreenToggle {
    fn with_fullscreen(&mut self, fullscreen: bool);
}

impl FullscreenToggle for BTerm {
    fn with_fullscreen(&mut self, _fullscreen: bool) {
        // NOTA: bracket-lib 0.8 no expone el cambio de pantalla completa en BTerm directamente.
    }
}

struct State {
    pub run_state: RunState,
    pub world_manager: WorldManager,
    pub time_state: TimeState,
    pub fullscreen: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // 4. Cierre del Sistema (Nivel más alto del tick)
        if ctx.key == Some(VirtualKeyCode::F4) && ctx.alt {
            ctx.quit();
            return;
        }

        // Global Input Detection
        if let Some(key) = ctx.key {
            match key {
                // Global Toggle Fullscreen (Spec-15)
                VirtualKeyCode::F11 => {
                    self.fullscreen = !self.fullscreen;
                    ctx.with_fullscreen(self.fullscreen);
                }
                VirtualKeyCode::Return if ctx.alt => {
                    self.fullscreen = !self.fullscreen;
                    ctx.with_fullscreen(self.fullscreen);
                }
                _ => {}
            }
        }

        if self.run_state == RunState::Quit {
            ctx.quit();
            return;
        }

        let new_runstate = match &self.run_state {
            RunState::MainMenu { selection } => states::main_menu::tick(ctx, &mut self.world_manager, selection.clone()),
            RunState::CharacterCreation => {
                ctx.set_active_console(1);
                ctx.cls();
                ctx.print_centered(25, "Character Creation");
                ctx.print_centered(27, "Press [M] to return to Main Menu");
                if let Some(VirtualKeyCode::M) = ctx.key {
                    Some(RunState::MainMenu { selection: states::MainMenuSelection::NewGame })
                } else {
                    None
                }
            }
            RunState::MapGen { phase, progress, phase_step } => states::map_gen_screen::tick(ctx, &mut self.world_manager, *phase, *progress, *phase_step),
            RunState::InGame | RunState::PlayerTurn | RunState::MonsterTurn => {
                states::ingame::tick(ctx, &mut self.world_manager, &mut self.time_state, self.run_state.clone())
            }

            RunState::Laboratory => states::laboratory::tick(ctx, &mut self.world_manager),
            RunState::MapInspector { zoom, cursor } => states::map_inspector::tick(ctx, &mut self.world_manager, *zoom, *cursor),
            RunState::Options { selection } => states::options::tick(ctx, *selection),
            RunState::PauseMenu { selection } => states::pause_menu::tick(ctx, &mut self.world_manager, *selection),
            RunState::Quit => None, 
        };

        if let Some(new_state) = new_runstate {
            self.run_state = new_state;
        }
    }
}

fn main() -> BError {
    let settings = Settings::load();
    let (width, height) = settings.get_dimensions();

    let context = BTermBuilder::new()
        .with_title("Rogue-Evolution")
        .with_dimensions(width, height)
        .with_tile_dimensions(8, 16)
        .with_font("vga8x16.png", 8, 16)
        .with_fullscreen(settings.fullscreen)
        .with_advanced_input(true)
        .with_fps_cap(60.0)
        .with_simple_console(width, height, "vga8x16.png")
        .with_simple_console_no_bg(width, height, "vga8x16.png")
        .build()?;

    let gs = State {
        run_state: RunState::MainMenu { selection: states::MainMenuSelection::NewGame },
        world_manager: WorldManager::new(width as i32, height as i32),
        time_state: TimeState::new(),
        fullscreen: settings.fullscreen,
    };
    main_loop(context, gs)
}
