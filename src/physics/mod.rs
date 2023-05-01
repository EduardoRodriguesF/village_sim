pub mod components;
mod debug;
pub mod prelude;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                apply_direction,
                apply_velocity,
                dynamic_collision,
                collision,
                detect_stuck,
                unstuck,
            )
                .chain(),
        );

        if cfg!(debug_assertions) {
            app.add_system(debug::draw_colliders);
        }
    }
}
