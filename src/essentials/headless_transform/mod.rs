pub mod components;
pub mod prelude;
mod systems;
pub mod resources;
mod debug;

use bevy::prelude::*;
use systems::*;
use resources::*;

pub struct HeadlessPositionPlugin;

impl Plugin for HeadlessPositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PixelScale>().add_system_to_stage(CoreStage::PostUpdate, translate_transform);

        if cfg!(debug_assertions) {
            app.add_system(debug::manual_scale_change);
        }
    }
}
