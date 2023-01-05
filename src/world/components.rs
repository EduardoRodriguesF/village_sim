use super::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct NpcStats {
    pub speed: f32,
    pub guts: u8,
    pub agility: u8,
}

impl NpcStats {
    pub fn new(rng: &mut StdRng) -> Self {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let weights = values.iter().rev();

        let dist = WeightedIndex::new(weights).unwrap();

        Self {
            speed: rng.gen_range(1.0..1.3),
            guts: values[dist.sample(rng)],
            agility: values[dist.sample(rng)],
        }
    }
}

#[derive(Component)]
pub struct Entrance;
