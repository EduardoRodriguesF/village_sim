pub mod components;
pub mod prelude;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_direction)
            .add_system(dynamic_collision)
            .add_system(apply_velocity.after(dynamic_collision));
    }
}
