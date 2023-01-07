pub mod components;
pub mod prelude;
pub mod resources;
mod systems;

use prelude::*;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Weather>()
            .add_startup_system(create_walls)
            .add_startup_system(spawn_entities)
            .add_system(populate);
    }
}
