use super::prelude::*;

pub fn apply_activity_plan(
    mut commands: Commands,
    q_people: Query<(Entity, &ActivityPlan, &HeadlessTransform), Without<Busy>>,
    q_activities: Query<(&Activity, Option<&HeadlessTransform>)>,
) {
    for (entity, plan, transform) in q_people.iter() {
        let mut entity = commands.entity(entity);
        entity.remove::<ActivityPlan>();

        if let Ok((activity, maybe_activity_transform)) = q_activities.get(plan.activity) {
            let location = match maybe_activity_transform {
                Some(transform) => Vec2::new(transform.translation.x, transform.translation.y),
                None => Vec2::new(transform.translation.x, transform.translation.y),
            };

            entity.insert(Busy::from_location(location, activity.avg_time_in_seconds));
        }
    }
}

pub fn go_to_activity(
    mut commands: Commands,
    q_people: Query<(Entity, &Busy), Without<DestinationNode>>,
) {
    for (entity, busy) in q_people.iter() {
        if let Some(location) = busy.location {
            let node = vec2_to_node(&location);

            commands.entity(entity).insert(DestinationNode(node));
        }
    }
}

pub fn do_activity(
    mut commands: Commands,
    time: Res<Time>,
    mut q_people: Query<(Entity, &HeadlessTransform, &mut Busy)>,
) {
    for (entity, transform, mut busy) in q_people.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);

        // Do not consider as doing activity if too far away.
        if let Some(location) = busy.location {
            if position.distance(location) > 16. {
                continue;
            }
        }

        if busy.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<Busy>();
        }
    }
}

pub fn follow_routine(
    mut commands: Commands,
    mut q_people: Query<(Entity, &mut Routine), Without<Busy>>,
) {
    for (entity, mut routine) in q_people.iter_mut() {
        let mut entity = commands.entity(entity);
        let total_activities = routine.activities.len();

        if total_activities > 0 {
            if let Some(item) = routine.next() {
                if let Some(activity) = item.activity {
                    entity.insert(ActivityPlan { activity });
                } else if let Some(busy) = item.busy {
                    entity.insert(busy);
                }
            } else {
                entity.remove::<Routine>();
            }
        }
    }
}
