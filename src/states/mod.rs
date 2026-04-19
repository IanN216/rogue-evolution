pub mod main_menu;
pub mod map_gen_screen;
pub mod ingame;
pub mod laboratory;
pub mod map_inspector;
pub mod options;
pub mod pause_menu;
pub mod character_creation;

#[derive(PartialEq, Clone, Debug)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame { selection: usize, cached_saves: Vec<String> },
    ConfirmDelete { selection: usize, cached_saves: Vec<String> },
    Laboratory,
    Options,
    Quit,
}

#[derive(PartialEq, Clone)]
pub enum RunState {
    MainMenu { selection: MainMenuSelection },
    CharacterCreation { selection: usize },
    MapGen { phase: usize, progress: f32, phase_step: usize },
    InGame,
    PlayerTurn,
    MonsterTurn,
    Laboratory,
    MapInspector { zoom: f32, cursor: (i32, i32) },
    Options { selection: usize },
    PauseMenu { selection: usize },
    Quit,
}
