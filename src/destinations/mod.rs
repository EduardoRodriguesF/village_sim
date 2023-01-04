pub mod components;
pub mod prelude;
mod systems;

use prelude::*;
use systems::*;

pub struct DestinationsPlugin;

impl Plugin for DestinationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(reconsider_path)
            .add_system(determine_instructions)
            .add_system(follow_instructions);
    }
}
