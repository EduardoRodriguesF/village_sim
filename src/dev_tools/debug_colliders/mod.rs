use crate::movement::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

const BOX_COLOR: Color = Color::RED;

pub struct DebugCollidersPlugin;

impl Plugin for DebugCollidersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_colliders);
    }
}

fn draw_colliders(mut lines: ResMut<DebugLines>, q_colliders: Query<(&HeadlessTransform, &Collider)>) {
    for (transform, collider) in q_colliders.iter() {
        let pos = to_vec2(&transform.translation);
        draw_rectangle(&mut lines, &pos, &collider.size, BOX_COLOR);
    }
}

