use crate::prelude::*;
use bevy::prelude::*;

pub fn translate_transform(
    scale: Res<PixelScale>,
    mut query: Query<(&HeadlessTransform, &mut Transform)>,
) {
    let scale = scale.extended_splat();

    for (headless, mut transform) in query.iter_mut() {
        transform.translation = (headless.translation * scale).round();
        transform.scale = headless.scale * scale;
        transform.rotation = headless.rotation;
    }
}

pub fn transition_scale(
    mut scale: ResMut<PixelScale>,
    target_scale: Res<TargetScale>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    if target_scale.value != scale.0 {
        let prev = scale.0;
        scale.0 = approach(scale.0, target_scale.value, target_scale.speed);

        if let Ok(mut transform) = q_camera.get_single_mut() {
            let delta = scale.0 - prev;
            let half_width = SCREEN_WIDTH as f32 / 2.;
            let half_height = SCREEN_HEIGHT as f32 / 2.;

            transform.translation.x += delta * half_width;
            transform.translation.y += delta * half_height;
        }
    }
}
