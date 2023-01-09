pub mod weather_tools;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;
use weather_tools::*;

pub struct WorldDebugPlugins;

impl PluginGroup for WorldDebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(WeatherToolsPlugin)
    }
}
