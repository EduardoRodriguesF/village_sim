pub mod cursor;
pub mod headless_transform;
pub mod prelude;
pub mod safe_despawn;
pub mod seed;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use cursor::CursorPlugin;
use headless_transform::HeadlessPositionPlugin;
use safe_despawn::SafeDespawnPlugin;
use seed::SeedPlugin;

pub struct EssentialPlugins;

impl PluginGroup for EssentialPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CursorPlugin)
            .add(HeadlessPositionPlugin)
            .add(SafeDespawnPlugin)
            .add(SeedPlugin)
    }
}
