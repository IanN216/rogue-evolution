pub mod main_menu;
pub mod map_gen_screen;
pub mod ingame;
pub mod laboratory;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    MainMenu { selection: main_menu::MainMenuSelection },
    CharacterCreation,
    MapGen,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    Laboratory,
}
