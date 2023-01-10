use crate::prelude::*;
use bevy::prelude::*;

pub fn translate_transform(scale: Res<PixelScale>, mut query: Query<(&HeadlessTransform, &mut Transform)>) {
    let scale = scale.extended_splat();

    for (headless, mut transform) in query.iter_mut() {
        transform.translation = (headless.translation * scale).round();
        transform.scale = headless.scale * scale;
        transform.rotation = headless.rotation;
    }
}
