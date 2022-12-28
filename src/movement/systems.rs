use super::prelude::*;
use crate::headless_transform::components::*;
use crate::world::prelude::*;
use crate::FPS;

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
