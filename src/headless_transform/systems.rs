use super::components::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn translate_transform(mut query: Query<(&HeadlessTransform, &mut Transform)>) {
    const DIMENSIONAL_SCALE: Vec3 = Vec3::splat(SCALE);

    for (headless, mut transform) in query.iter_mut() {
        transform.translation = (headless.translation * DIMENSIONAL_SCALE).round();
        transform.scale = headless.scale * DIMENSIONAL_SCALE;
        transform.rotation = headless.rotation;
    }
}
