pub mod components;
pub mod prelude;
mod systems;

use prelude::*;
use systems::*;

pub struct NpcToolsPlugin;

impl Plugin for NpcToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_tracking_text)
            .add_system(attach_log)
            .add_system(create_npc)
            .add_system(trace_path)
            .add_system(update_position_text)
            .add_system(update_velocity_text)
            .add_system(update_routine_len_text)
            .add_system(update_routine_idx_text)
            .add_system(update_routine_loop_text)
            .add_system(update_activity_location_text)
            .add_system(update_activity_timer_text);
    }
}
