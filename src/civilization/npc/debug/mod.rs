pub mod npc_counting;
pub mod npc_tools;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;
use npc_counting::*;
use npc_tools::*;

pub struct NpcDebugPlugins;

impl PluginGroup for NpcDebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NpcToolsPlugin)
            .add(NpcCountingPlugin)
    }
}
