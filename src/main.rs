mod movement;
mod headless_transform;
mod world;

mod prelude {
    pub use crate::headless_transform::components::*;
    pub use bevy::prelude::*;

    pub const SCALE: f32 = 1.;
}

use headless_transform::HeadlessPositionPlugin;
use prelude::*;
use world::WorldPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HeadlessPositionPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(MovementPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
