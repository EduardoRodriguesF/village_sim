use super::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub one_time_speed: Option<Vec2>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, ..default() }
    }

    pub fn into_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Component)]
pub struct MovementIntention {
    pub x: f32,
    pub y: f32,
}

impl MovementIntention {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0., 0.)
    }
}

#[derive(Component, Default)]
pub struct Collider {
    pub size: Vec2,
    pub anchor: Anchor,
}

impl Collider {
    pub fn new(size: Vec2, anchor: Anchor) -> Self {
        Self { size, anchor }
    }
}
