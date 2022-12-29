pub mod prelude;
pub mod shapes;

use bevy::prelude::*;

pub fn to_vec3(vec2: &Vec2) -> Vec3 {
    Vec3::new(vec2.x, vec2.y, 1.)
}

pub fn to_vec2(vec3: &Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}
