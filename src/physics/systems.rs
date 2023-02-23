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

/// Handles collision between moving and static entities.
pub fn collision(
    mut q_movers: Query<(&mut Velocity, &HeadlessTransform, &Collider)>,
    q_statics: Query<(&HeadlessTransform, &Collider), Without<Velocity>>,
) {
    for (mut velocity, mover_transform, mover_collider) in q_movers.iter_mut() {
        for (static_transform, static_collider) in q_statics.iter() {
            // Colliders position
            let mover_pos = mover_transform.translation.truncate() + mover_collider.offset();
            let static_projection =
                static_transform.translation.truncate() + static_collider.offset();

            // Horizontal collision
            while collide(
                (mover_pos + velocity.as_vec2() * Vec2::X).extend(1.),
                mover_collider.size,
                static_projection.extend(1.),
                static_collider.size,
            )
            .is_some()
                // Prevent infinite loop when entity is stuck
                && velocity.x != 0.
            {
                // Find mininum velocity before colliding
                velocity.x = approach(velocity.x, 0., 0.25);
            }

            // Vertical collision
            while collide(
                (mover_pos + velocity.as_vec2() * Vec2::Y).extend(1.),
                mover_collider.size,
                static_projection.extend(1.),
                static_collider.size,
            )
            .is_some()
                && velocity.y != 0.
            {
                velocity.y = approach(velocity.y, 0., 0.25);
            }
        }
    }
}

pub fn detect_stuck(
    mut commands: Commands,
    q_movers: Query<(Entity, &HeadlessTransform, &Collider), With<Velocity>>,
    q_walls: Query<(Entity, &HeadlessTransform, &Collider), Without<Velocity>>,
) {
    for (mover_entity, mover_transform, mover_collider) in q_movers.iter() {
        for (wall_entity, wall_transform, wall_collider) in q_walls.iter() {
            if collide(
                mover_transform.translation + mover_collider.offset().extend(0.),
                mover_collider.size,
                wall_transform.translation + wall_collider.offset().extend(0.),
                wall_collider.size,
            )
            .is_some()
            {
                commands.entity(mover_entity).insert(Stuck(wall_entity));
            }
        }
    }
}

pub fn unstuck(
    mut seed: ResMut<Seed>,
    mut commands: Commands,
    mut q_stucks: Query<(Entity, &mut HeadlessTransform, &Collider, &Stuck), With<Velocity>>,
    q_walls: Query<(&HeadlessTransform, &Collider), Without<Velocity>>,
) {
    let mut directions = vec![
        Vec2::new(1., 0.),
        Vec2::new(0., 1.),
        Vec2::new(-1., 0.),
        Vec2::new(0., -1.),
    ];

    // Prevent stuck loops
    directions.shuffle(&mut seed.rng);

    for (entity, mut transform, collider, stuck) in q_stucks.iter_mut() {
        if let Ok((wall_transform, wall_collider)) = q_walls.get(stuck.0) {
            let mut i = 0;

            'outer: loop {
                for dir in directions.iter() {
                    // Continuously increase direction range until unstuck.
                    let dir = *dir * Vec2::splat(i as f32);

                    if collide(
                        (transform.translation.truncate() + collider.offset() + dir).extend(1.),
                        collider.size,
                        wall_transform.translation + wall_collider.offset().extend(0.),
                        wall_collider.size,
                    )
                    .is_none()
                    {
                        commands.entity(entity).remove::<Stuck>();

                        // Apply translation to unstuck.
                        transform.translation += dir.extend(0.);

                        break 'outer;
                    }
                }

                i += 1;
            }
        }
    }
}
