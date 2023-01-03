use super::prelude::*;

#[derive(Component, Debug, Default)]
pub struct NpcStats {
    pub speed: f32,
    pub courage: u8,
}

#[derive(Component)]
pub struct Entrance;
