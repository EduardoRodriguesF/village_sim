pub mod physics;
pub mod npc;

use bevy::prelude::*;
use physics::PhysicsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin);
    }
}
