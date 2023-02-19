pub mod npc;
pub mod prelude;
pub mod world;
pub mod player;

use crate::prelude::*;
use npc::*;
use world::*;
use player::*;

pub struct CivilizationPlugin;

impl Plugin for CivilizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin).add_plugin(NpcPlugin).add_plugin(PlayerPlugin);
    }
}
