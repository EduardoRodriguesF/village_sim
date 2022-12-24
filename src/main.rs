mod destinations;
mod headless_transform;
mod movement;
mod world;

mod prelude {
    pub use crate::headless_transform::components::*;
    pub use bevy::prelude::*;

    pub const SCALE: f32 = 1.;
}

use destinations::DestinationsPlugin;
use headless_transform::HeadlessPositionPlugin;
use movement::MovementPlugin;
use prelude::*;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HeadlessPositionPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(DestinationsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
