pub mod prelude;
pub mod components;
mod systems;

use prelude::*;
use systems::*;

pub struct DestinationsPlugin;

impl Plugin for DestinationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(determine_instructions).add_system(follow_instructions);
    }
}
