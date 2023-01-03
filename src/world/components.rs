use super::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[derive(Component, Debug, Default)]
pub struct NpcStats {
    pub speed: f32,
    pub courage: u8,
}

impl NpcStats {
    pub fn new(rng: &mut StdRng) -> Self {
        let values = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let weights = values.iter().rev();

        let dist = WeightedIndex::new(weights).unwrap();

        Self {
            speed: rng.gen_range(1.0..1.3),
            courage: values[dist.sample(rng)],
        }
    }
}

#[derive(Component)]
pub struct Entrance;
