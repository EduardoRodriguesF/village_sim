pub mod components;
mod debug;
pub mod prelude;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct HeadlessPositionPlugin;

impl Plugin for HeadlessPositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PixelScale>()
            .init_resource::<TargetScale>()
            .add_system(translate_transform.in_base_set(CoreSet::PostUpdate))
            .add_system(transition_scale);

        if cfg!(debug_assertions) {
            app.add_system(debug::manual_scale_change)
                .add_system(debug::notify_scale_change);
        }
    }
}
