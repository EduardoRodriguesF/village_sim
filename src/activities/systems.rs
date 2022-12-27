use super::prelude::*;
use crate::world::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

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

pub fn search_activity(
    mut commands: Commands,
    mut seed: ResMut<Seed>,
    q_people: Query<(Entity, &SearchingActivity, &HeadlessTransform), Without<Busy>>,
    q_activities: Query<(Entity, &Identifier, &HeadlessTransform), With<Activity>>,
) {
    for (entity, search, transform) in q_people.iter() {
        let mut matching_activities = q_activities
            .iter()
            .filter_map(
                |(entity, identifier, transform)| match search.eq(identifier) {
                    true => Some((entity, transform)),
                    false => None,
                },
            )
            .collect::<Vec<(Entity, &HeadlessTransform)>>();

        if matching_activities.len() > 0 {
            matching_activities.sort_by(|(_, a_transform), (_, b_transform)| {
                let a_distance = transform.translation.distance(a_transform.translation);
                let b_distance = transform.translation.distance(b_transform.translation);

                a_distance.partial_cmp(&b_distance).unwrap()
            });

            // Nearest will be preferred
            let weights: Vec<usize> = matching_activities
                .iter()
                .enumerate()
                .rev()
                .map(|(idx, _)| (idx * idx) + 1)
                .collect();

            let dist = WeightedIndex::new(weights).unwrap();
            let (chosen, _transform) = matching_activities[dist.sample(&mut seed.rng)];

            let activity = commands.entity(chosen.to_owned()).id();

            commands.entity(entity).insert(ActivityPlan { activity });
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
                } else if let Some(busy) = &item.busy {
                    entity.insert(busy.to_owned());
                } else if let Some(search) = &item.search {
                    entity.insert(SearchingActivity(Identifier(search.to_string())));
                }
            } else {
                entity.remove::<Routine>();
            }
        }
    }
}
