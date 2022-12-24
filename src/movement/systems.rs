use super::prelude::*;
use crate::headless_transform::components::*;

pub fn apply_velocity(mut query: Query<(&mut HeadlessTransform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
