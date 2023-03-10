pub mod benchmark;
pub mod prelude;
pub mod time_manipulation;

use bevy_prototype_debug_lines::DebugLinesPlugin;
use prelude::*;

pub struct DevToolsPlugins;

impl PluginGroup for DevToolsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BenchmarkPlugin)
            .add(TimeManipulationPlugin)
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(DebugLinesPlugin::default())
    }
}
