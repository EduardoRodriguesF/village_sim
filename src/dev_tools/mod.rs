pub mod benchmark;
pub mod prelude;
pub mod seed_info;
pub mod npc_tools;

use prelude::*;

pub struct DevToolsPlugins;

impl PluginGroup for DevToolsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BenchmarkPlugin)
            .add(SeedInfoPlugin)
            .add(NpcToolsPlugin)
            .add(FrameTimeDiagnosticsPlugin::default())
    }
}
