pub mod main_menu;
pub mod map_gen_screen;
pub mod ingame;
pub mod laboratory;
pub mod map_inspector;

#[derive(PartialEq, Clone, Debug)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame { selection: usize, cached_saves: Vec<String> },
    ConfirmDelete { selection: usize, cached_saves: Vec<String> },
    Laboratory,
    Quit,
}

#[derive(PartialEq, Clone)]
pub enum RunState {
    MainMenu { selection: MainMenuSelection },
    CharacterCreation,
    MapGen,
    InGame,
    PlayerTurn,
    MonsterTurn,
    Laboratory,
    MapInspector { zoom: f32, cursor: (i32, i32) },
}
