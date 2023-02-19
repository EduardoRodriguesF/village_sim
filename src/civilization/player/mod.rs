pub mod components;
mod systems;

use crate::prelude::*;
use systems::*;

pub mod prelude {
    pub use super::components::*;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}
