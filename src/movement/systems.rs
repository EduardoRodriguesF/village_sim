use super::prelude::*;
use crate::headless_transform::components::*;
use crate::world::prelude::*;
use crate::FPS;
use bevy::sprite::collide_aabb::collide;

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut HeadlessTransform, &mut Velocity)>) {
    let delta = time.delta_seconds() * FPS;

    for (mut transform, mut velocity) in query.iter_mut() {
        let mut translation_change = Vec3::new(velocity.x, velocity.y, 0.);

        if let Some(one_time_speed) = velocity.one_time_speed {
            translation_change.x += one_time_speed.x;
            translation_change.y += one_time_speed.y;

            velocity.one_time_speed = None;
        }

        transform.translation += translation_change * delta;
    }
}

pub fn apply_direction(
    mut commands: Commands,
    mut query: Query<(Entity, &MovementIntention, &NpcStats)>,
) {
    for (entity, delta, stats) in query.iter_mut() {
        let mut entity = commands.entity(entity);
        entity.remove::<MovementIntention>();

        let velocity = Velocity::new(delta.x * stats.speed, delta.y * stats.speed);

        entity.insert(velocity);
    }
}

pub fn dynamic_collision(mut q_colliders: Query<(&mut Velocity, &HeadlessTransform, &Collider)>) {
    let mut combinations = q_colliders.iter_combinations_mut();
    while let Some([a1, a2]) = combinations.fetch_next() {
        let (mut a1_velocity, a1_transform, a1_collider) = a1;
        let (mut a2_velocity, a2_transform, a2_collider) = a2;

        let a1_projection = to_vec2(&a1_transform.translation) + a1_velocity.as_vec2();
        let a2_projection = to_vec2(&a2_transform.translation) + a2_velocity.as_vec2();

        if collide(
            to_vec3(&a1_projection),
            a1_collider.size,
            to_vec3(&a2_projection),
            a2_collider.size,
        )
        .is_some()
        {
            let dir = Vec2::normalize(a2_projection - a1_projection);
            let mut forces = Vec2::normalize(a2_velocity.as_vec2() - a1_velocity.as_vec2());

            if forces.is_nan() {
                forces = Vec2::splat(0.);
            }

            a1_velocity.one_time_speed = Some((dir - forces) * -1.);
            a2_velocity.one_time_speed = Some(dir - forces);
        }
    }
}
