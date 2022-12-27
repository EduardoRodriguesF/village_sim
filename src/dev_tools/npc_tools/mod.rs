pub mod components;
pub mod prelude;
mod systems;

use prelude::*;
use systems::*;

pub struct NpcToolsPlugin;

impl Plugin for NpcToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attach_log);
    }
}
