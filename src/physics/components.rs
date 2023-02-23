use crate::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub one_time_speed: Option<Vec2>,
}

impl Velocity {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            one_time_speed: None,
        }
    }

    pub const fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MovementIntention {
    pub x: f32,
    pub y: f32,
}

impl MovementIntention {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Default)]
pub struct Collider {
    pub size: Vec2,
    pub anchor: Anchor,
}

impl Collider {
    pub const fn new(size: Vec2, anchor: Anchor) -> Self {
        Self { size, anchor }
    }

    pub fn offset(&self) -> Vec2 {
        self.size * self.anchor.as_vec()
    }
}
