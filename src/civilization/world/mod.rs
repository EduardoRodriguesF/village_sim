pub mod components;
mod debug;
pub mod prelude;
pub mod resources;
mod systems;

use crate::prelude::*;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Weather>()
            .add_startup_system(create_walls)
            .add_startup_system(spawn_entities)
            .add_system(populate);

        if cfg!(debug_assertions) {
            app.add_plugins(debug::WorldDebugPlugins);
        }
    }
}
