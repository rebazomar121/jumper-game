mod player;
mod obstacle;
mod state;
mod mode;

mod prelude {
    pub use crate::player::*;
    pub use crate::obstacle::*;
    pub use crate::state::*;
    pub use crate::mode::*;
    pub use bracket_lib::prelude::*;
}

use crate::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rebaz Omar & zhin rzgar")
        .build()?;

    main_loop(context, State::new())
}
