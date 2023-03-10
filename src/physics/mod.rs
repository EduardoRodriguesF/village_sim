pub mod components;
mod debug;
pub mod prelude;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_direction.before("collision"))
            .add_system_set(
                SystemSet::new()
                    .label("collision")
                    .with_system(dynamic_collision)
                    .with_system(collision.after(dynamic_collision))
                    .with_system(detect_stuck)
                    .with_system(unstuck),
            )
            .add_system(apply_velocity.after("collision"));

        if cfg!(debug_assertions) {
            app.add_system(debug::draw_colliders);
        }
    }
}
