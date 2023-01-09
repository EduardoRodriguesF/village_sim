pub mod components;
pub mod prelude;
mod systems;

use crate::prelude::*;
use systems::*;

pub struct ActivitiesPlugin;

impl Plugin for ActivitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(react)
            .add_system(have_reaction)
            .add_system(apply_activity_plan)
            .add_system(search_activity)
            .add_system(do_activity)
            .add_system(follow_routine);
    }
}
