use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn draw_rectangle(lines: &mut DebugLines, pos: &Vec2, size: &Vec2, color: Color) {
    let mut draw = |start: Vec2, end: Vec2| {
        lines.line_colored(start.extend(1.), end.extend(1.), 0., color);
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
