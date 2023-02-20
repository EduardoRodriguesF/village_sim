use super::prelude::*;
use crate::npc::prelude::*;
use crate::FPS;

/// Translates entities position based on its current `Velocity`.
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

/// Uses `MovementIntention` to create new `Velocity` value.
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

/// Handles collision between moving entities.
pub fn dynamic_collision(mut q_colliders: Query<(&mut Velocity, &HeadlessTransform, &Collider)>) {
    let mut combinations = q_colliders.iter_combinations_mut();
    while let Some([a1, a2]) = combinations.fetch_next() {
        let (mut a1_velocity, a1_transform, a1_collider) = a1;
        let (mut a2_velocity, a2_transform, a2_collider) = a2;

        let a1_projection =
            a1_transform.translation.truncate() + a1_velocity.as_vec2() + a1_collider.offset();
        let a2_projection =
            a2_transform.translation.truncate() + a2_velocity.as_vec2() + a2_collider.offset();

        if collide(
            a1_projection.extend(1.),
            a1_collider.size,
            a2_projection.extend(1.),
            a2_collider.size,
        )
        .is_some()
        {
            let dir = Vec2::normalize_or_zero(a2_projection - a1_projection);

            a1_velocity.one_time_speed = Some(dir * -1.);
            a2_velocity.one_time_speed = Some(dir);
        }
    }
}
