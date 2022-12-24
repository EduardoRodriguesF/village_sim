mod systems;
pub mod resources;

use bevy::prelude::*;
use systems::*;

pub mod prelude {
    pub use super::resources::*;
}

use prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>().add_startup_system(setup);

        if cfg!(debug_assertions) {
            app.add_startup_system(initial_debug);
        }
    }
}
