pub mod components;
pub mod map;
pub mod prelude;
pub mod resources;
mod systems;

use prelude::*;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>()
            .insert_resource(Map::from_ldtk("data/village.ldtk"))
            .add_startup_system(setup)
            .add_startup_system(create_walls)
            .add_startup_system(spawn_entities);
    }
}
