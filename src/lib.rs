pub mod dev_tools;
pub mod essentials;
pub mod map;
pub mod npc;
pub mod physics;
pub mod prelude;
pub mod utils;
pub mod world;

use dev_tools::*;
use essentials::*;
use npc::*;
use physics::*;
use prelude::*;
use world::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EssentialPlugins)
            .insert_resource(Map::from_ldtk("data/village.ldtk"))
            .add_plugin(PhysicsPlugin)
            .add_plugin(NpcPlugin)
            .add_plugin(WorldPlugin)
            .add_startup_system(setup)
            .add_system(camera_movement);

        if cfg!(debug_assertions) {
            app.add_plugins(DevToolsPlugins);
        }
    }
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
