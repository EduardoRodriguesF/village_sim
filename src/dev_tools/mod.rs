pub mod benchmark;
pub mod prelude;

use prelude::*;

pub struct DevToolsPlugins;

impl PluginGroup for DevToolsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BenchmarkPlugin)
            .add(FrameTimeDiagnosticsPlugin::default())
    }
}
