pub mod components;
pub mod prelude;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct HeadlessPositionPlugin;

impl Plugin for HeadlessPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, translate_transform);
    }
}
