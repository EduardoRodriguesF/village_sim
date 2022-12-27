pub mod benchmark;
pub mod npc_counting;
pub mod npc_tools;
pub mod prelude;
pub mod seed_info;

use prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub struct DevToolsPlugins;

impl PluginGroup for DevToolsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BenchmarkPlugin)
            .add(SeedInfoPlugin)
            .add(NpcToolsPlugin)
            .add(NpcCountingPlugin)
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(DebugLinesPlugin::default())
    }
}
