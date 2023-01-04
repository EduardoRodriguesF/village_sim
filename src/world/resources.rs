use super::prelude::*;
use rand::prelude::*;

#[derive(Resource, Debug)]
pub struct Seed {
    pub rng: StdRng,
    pub key: u64,
}

impl Default for Seed {
    fn default() -> Self {
        let key: u64 = rand::random();
        let rng = StdRng::seed_from_u64(key);

        Seed { rng, key }
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub enum Weather {
    #[default]
    Clear,
    Rain,
}

impl Weather {
    /// Returns `true` if the weather is [`Clear`].
    ///
    /// [`Clear`]: Weather::Clear
    #[must_use]
    pub fn is_clear(&self) -> bool {
        matches!(self, Self::Clear)
    }
}
