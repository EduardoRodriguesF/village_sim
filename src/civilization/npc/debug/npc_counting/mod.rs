pub mod components;
pub mod prelude;
pub mod resources;
mod systems;

use prelude::*;
use systems::*;

pub struct NpcCountingPlugin;

impl Plugin for NpcCountingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NpcCount>()
            .add_startup_system(setup)
            .add_system(count_npcs)
            .add_system(update_total_text)
            .add_system(update_visible_text);
    }
}
