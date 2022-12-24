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
            .insert_resource(Map::new(vec![
                "1111111111111111111",
                "1111111111111111111",
                "1111111111111111111",
                "1111111xxxxxxxxxx11",
                "1111111xxxxxxxxxx11",
                "1111111xxxxxxxxxx11",
                "1111111xxxxxxxxxx11",
                "1111111111111111111",
            ]))
            .add_startup_system(setup);

        if cfg!(debug_assertions) {
            app.add_startup_system(initial_debug);
        }
    }
}
