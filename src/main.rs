mod states;
mod core;
mod components;
mod systems;
mod utils;

use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Rogue-Evolution: Inicializado (Celeron N2806 Optimized)");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rogue-Evolution")
        .build()?;

    let gs = State {};
    main_loop(context, gs)
}
