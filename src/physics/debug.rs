use super::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

const BOX_COLOR: Color = Color::RED;

pub fn draw_colliders(
    mut lines: ResMut<DebugLines>,
    scale: Res<PixelScale>,
    q_colliders: Query<(&Transform, &Collider)>,
) {
    let scale = scale.splat();

    for (transform, collider) in q_colliders.iter() {
        let size = collider.size * scale;
        let pos = transform.translation.truncate() + collider.offset();

        draw_rectangle(&mut lines, &pos, &size, BOX_COLOR);
    }
}
