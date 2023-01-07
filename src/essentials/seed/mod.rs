pub mod prelude;
pub mod resources;

use bevy::prelude::*;
use resources::*;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>();
    }
}
