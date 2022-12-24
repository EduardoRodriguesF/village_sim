mod destinations;
mod headless_transform;
mod movement;
mod world;

mod prelude {
    pub use crate::headless_transform::components::*;
    pub use bevy::prelude::*;

    pub const SCALE: f32 = 1.;
    pub const SCREEN_WIDTH: i32 = 1000;
    pub const SCREEN_HEIGHT: i32 = 602;
}

use destinations::DestinationsPlugin;
use headless_transform::HeadlessPositionPlugin;
use movement::MovementPlugin;
use prelude::*;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH as f32,
                height: SCREEN_HEIGHT as f32,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HeadlessPositionPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(DestinationsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32, 1.),
        ..default()
    });
}
