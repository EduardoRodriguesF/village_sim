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

fn draw_colliders(mut lines: ResMut<DebugLines>, q_colliders: Query<(&HeadlessTransform, &Collider, &Sprite)>) {
    for (transform, collider, _sprite) in q_colliders.iter() {
        let pos = to_vec2(&transform.translation);
        draw_rectangle(&mut lines, &pos, &collider.size);
    }
}

fn draw_rectangle(lines: &mut DebugLines, pos: &Vec2, size: &Vec2) {
    let mut draw = |start: Vec2, end: Vec2| {
        lines.line_colored(to_vec3(&start), to_vec3(&end), 0., BOX_COLOR);
    };

    let left = pos.x;
    let top = pos.y;
    let right = pos.x + size.y;
    let bottom = pos.y + size.y;

    let top_left = Vec2::new(left, top);
    let top_right = Vec2::new(right, top);
    let bottom_left = Vec2::new(left, bottom);
    let bottom_right = Vec2::new(right, bottom);

    draw(top_left, top_right); // Top
    draw(top_right, bottom_right); // Right
    draw(bottom_right, bottom_left); // Bottom
    draw(bottom_left, top_left); // Left
}
