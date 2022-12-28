use super::prelude::*;
use crate::headless_transform::components::*;
use crate::world::prelude::*;

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut HeadlessTransform, &Velocity)>) {
    let delta = time.delta_seconds() * 60.;

    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * delta;
        transform.translation.y += velocity.y * delta;
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
