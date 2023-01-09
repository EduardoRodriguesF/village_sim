pub mod components;
mod debug;
pub mod prelude;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_direction.before(dynamic_collision))
            .add_system(dynamic_collision.before(apply_velocity))
            .add_system(apply_velocity);

        if cfg!(debug_assertions) {
            app.add_system(debug::draw_colliders);
        }
    }
}
