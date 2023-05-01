pub mod components;
pub mod prelude;
mod systems;

use prelude::*;
use systems::*;

pub struct DestinationsPlugin;

impl Plugin for DestinationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            check_path_conditions_change,
            reconsider_path,
            determine_instructions,
            follow_instructions,
        ));
    }
}
