pub mod npc;
pub mod prelude;
pub mod world;

use crate::prelude::*;
use npc::*;
use world::*;

pub struct CivilizationPlugin;

impl Plugin for CivilizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin).add_plugin(NpcPlugin);
    }
}
