mod dev_tools;
pub mod essentials;
pub mod map;
mod npc;
mod physics;
pub mod utils;
mod world;

mod prelude {
    pub use crate::essentials::prelude::*;
    pub use crate::map::*;
    pub use crate::physics::prelude::*;
    pub use crate::utils::prelude::*;
    pub use crate::world::prelude::*;
    pub use bevy::prelude::*;

    pub const FPS: f32 = 60.;
    pub const SCALE: f32 = 1.;
    pub const SCREEN_WIDTH: i32 = 960;
    pub const SCREEN_HEIGHT: i32 = 592;
}

use dev_tools::DevToolsPlugins;
use essentials::EssentialPlugins;
use npc::NpcPlugin;
use physics::PhysicsPlugin;
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
    .add_plugins(EssentialPlugins)
    .insert_resource(Map::from_ldtk("data/village.ldtk"))
    .add_plugin(PhysicsPlugin)
    .add_plugin(NpcPlugin)
    .add_plugin(WorldPlugin)
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
