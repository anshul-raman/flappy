use bracket_lib::prelude::*;

mod constans;
mod obstacle;
mod player;
mod state;

use state::State;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50().with_title("Flappy @").build()?;

    main_loop(ctx, State::new())
}
