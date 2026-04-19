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
use crate::utils::ui_constants::*;

struct State {
    pub run_state: RunState,
    pub world_manager: WorldManager,
    pub time_state: TimeState,
    pub fullscreen: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Cierre del Sistema (Nivel más alto del tick)
        if ctx.key == Some(VirtualKeyCode::F4) && ctx.alt {
            ctx.quit();
            return;
        }

        // Global Input Detection
        if let Some(key) = ctx.key {
            match key {
                // Global Toggle Fullscreen (Puramente informativo en runtime para v0.8)
                VirtualKeyCode::F11 | VirtualKeyCode::Return if ctx.alt => {
                    self.fullscreen = !self.fullscreen;
                    // El cambio real requiere reinicio para aplicarse en bracket-lib 0.8
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
            RunState::CharacterCreation { selection } => states::character_creation::tick(ctx, &mut self.world_manager, *selection),
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
    
    // Configuración de Consolas con Resolución Lógica Fija (UI Upscaling)
    let context = BTermBuilder::new()
        .with_title("Rogue-Evolution")
        .with_dimensions(LOGICAL_WIDTH, LOGICAL_HEIGHT)
        .with_tile_dimensions(8, 16)
        .with_font("vga8x16.png", 8, 16)
        .with_fullscreen(settings.fullscreen)
        .with_advanced_input(true)
        .with_fps_cap(60.0)
        // Capa 0: Terreno / Mundo
        .with_simple_console(LOGICAL_WIDTH, LOGICAL_HEIGHT, "vga8x16.png")
        // Capa 1: Entidades
        .with_simple_console_no_bg(LOGICAL_WIDTH, LOGICAL_HEIGHT, "vga8x16.png")
        // Capa 2: UI / Menús
        .with_simple_console_no_bg(LOGICAL_WIDTH, LOGICAL_HEIGHT, "vga8x16.png")
        .build()?;

    let gs = State {
        run_state: RunState::MainMenu { selection: states::MainMenuSelection::NewGame },
        world_manager: WorldManager::new(LOGICAL_WIDTH, LOGICAL_HEIGHT),
        time_state: TimeState::new(),
        fullscreen: settings.fullscreen,
    };
    main_loop(context, gs)
}
