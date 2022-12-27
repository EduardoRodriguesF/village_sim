pub mod benchmark;
pub mod prelude;
pub mod seed_info;

use prelude::*;

pub struct DevToolsPlugins;

impl PluginGroup for DevToolsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BenchmarkPlugin)
            .add(SeedInfoPlugin)
            .add(FrameTimeDiagnosticsPlugin::default())
    }
}
