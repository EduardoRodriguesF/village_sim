pub mod activities;
pub mod components;
mod debug;
pub mod destinations;
pub mod prelude;

use bevy::prelude::*;
use prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActivitiesPlugin)
            .add_plugin(DestinationsPlugin);

        if cfg!(debug_assertions) {
            app.add_plugins(debug::NpcDebugPlugins);
        }
    }
}
