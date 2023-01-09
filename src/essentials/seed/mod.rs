pub mod prelude;
pub mod resources;

use bevy::prelude::*;
use resources::*;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>();

        if cfg!(debug_assertions) {
            app.add_startup_system(log_seed);
        }
    }
}

fn log_seed(seed: Res<Seed>) {
    info!("Seed: {}", seed.key)
}
