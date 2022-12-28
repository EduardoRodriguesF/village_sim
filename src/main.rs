mod activities;
mod cursor;
mod destinations;
mod dev_tools;
mod headless_transform;
mod movement;
mod world;
pub mod utils;

mod prelude {
    pub use crate::headless_transform::components::*;
    pub use crate::utils::*;
    pub use bevy::prelude::*;

    pub const FPS: f32 = 60.;
    pub const SCALE: f32 = 1.;
    pub const SCREEN_WIDTH: i32 = 960;
    pub const SCREEN_HEIGHT: i32 = 592;
}

use activities::ActivitiesPlugin;
use cursor::CursorPlugin;
use destinations::DestinationsPlugin;
use dev_tools::DevToolsPlugins;
use headless_transform::HeadlessPositionPlugin;
use movement::MovementPlugin;
use prelude::*;
use world::WorldPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    .add_plugin(ActivitiesPlugin)
    .add_plugin(CursorPlugin)
    .add_startup_system(setup)
    .add_system(camera_movement);

    if cfg!(debug_assertions) {
        app.add_plugins(DevToolsPlugins);
    }

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32, 1.),
        ..default()
    });
}

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.get_single_mut().unwrap();

    let pressed_right = keyboard_input.pressed(KeyCode::Right) as i32;
    let pressed_left = keyboard_input.pressed(KeyCode::Left) as i32;
    let pressed_up = keyboard_input.pressed(KeyCode::Up) as i32;
    let pressed_down = keyboard_input.pressed(KeyCode::Down) as i32;

    // Integer direction of evaluated individual directions.
    let delta_x = pressed_right - pressed_left;
    let delta_y = pressed_up - pressed_down;

    transform.translation.x += 3. * delta_x as f32;
    transform.translation.y += 3. * delta_y as f32;
}
