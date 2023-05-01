pub mod components;
pub mod prelude;
mod systems;

use crate::prelude::*;
use systems::*;

pub struct ActivitiesPlugin;

impl Plugin for ActivitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            react,
            have_reaction,
            apply_activity_plan,
            search_activity,
            do_activity,
            follow_routine,
        ));
    }
}
