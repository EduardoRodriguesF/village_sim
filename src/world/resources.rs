use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource, Debug)]
pub struct Seed {
    pub rng: StdRng,
}

impl Default for Seed {
    fn default() -> Self {
        Seed {
            rng: StdRng::from_entropy(),
        }
    }
}
