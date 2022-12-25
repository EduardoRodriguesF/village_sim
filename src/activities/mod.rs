pub mod components;
pub mod prelude;
mod systems;

use prelude::*;
use systems::*;

pub struct ActivitiesPlugin;

impl Plugin for ActivitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(go_to_activity).add_system(do_activity).add_system(follow_routine);
    }
}
