use crate::world::prelude::*;
use bevy::prelude::*;

pub struct SeedInfoPlugin;

impl Plugin for SeedInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(log_seed);
    }
}

fn log_seed(seed: Res<Seed>) {
    info!("Seed: {}", seed.key)
}
