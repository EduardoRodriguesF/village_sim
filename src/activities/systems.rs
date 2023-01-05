use super::prelude::*;
use crate::world::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn react(time: Res<Time>, mut commands: Commands, mut query: Query<(Entity, &mut Reaction)>) {
    for (entity, mut reaction) in query.iter_mut() {
        if reaction.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<Reaction>();
        }
    }
}

pub fn have_reaction(
    mut seed: ResMut<Seed>,
    mut commands: Commands,
    q_npcs: Query<
        (Entity, &NpcStats),
        Or<(Added<ActivityPlan>, Added<SearchingActivity>, Added<Busy>)>,
    >,
) {
    for (entity, stats) in q_npcs.iter() {
        let base = 1. / stats.agility as f32 * 5.;
        let range = (base / 2.)..base;

        commands.entity(entity).insert(Reaction {
            timer: Timer::from_seconds(seed.rng.gen_range(range), TimerMode::Once),
        });
    }
}

pub fn apply_activity_plan(
    mut seed: ResMut<Seed>,
    mut commands: Commands,
    q_people: Query<
        (Entity, &ActivityPlan, &HeadlessTransform),
        (Without<Busy>, Without<Reaction>),
    >,
    q_activities: Query<(&Activity, Option<&HeadlessTransform>)>,
) {
    for (entity, plan, transform) in q_people.iter() {
        let mut entity = commands.entity(entity);

        if let Ok((activity, maybe_activity_transform)) = q_activities.get(plan.activity) {
            let location = match maybe_activity_transform {
                Some(transform) => {
                    let pos = transform.translation.truncate();
                    let half_size = activity.area.half_size();
                    let deviation = Vec2::new(
                        seed.rng.gen_range(-half_size.x..half_size.x),
                        seed.rng.gen_range(-half_size.y..half_size.y),
                    );

                    pos + deviation
                }
                None => transform.translation.truncate(),
            };

            entity.insert((
                Busy::from_location(location, activity.avg_time_in_seconds),
                DestinationPoint(location),
            ));
        }
    }
}

pub fn search_activity(
    mut commands: Commands,
    mut seed: ResMut<Seed>,
    q_people: Query<
        (Entity, &SearchingActivity, &HeadlessTransform),
        (Without<ActivityPlan>, Without<Reaction>),
    >,
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

        if !matching_activities.is_empty() {
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

pub fn do_activity(
    mut commands: Commands,
    time: Res<Time>,
    mut q_people: Query<
        (Entity, &HeadlessTransform, &mut Busy, &ActivityPlan),
        (Without<DestinationPoint>, Without<Reaction>),
    >,
    q_activities: Query<&Identifier, With<Activity>>,
) {
    for (entity, transform, mut busy, plan) in q_people.iter_mut() {
        let position = transform.translation.truncate();

        // Do not consider as doing activity if too far away.
        if let Some(location) = busy.location {
            if position.distance(location) > 16. {
                commands.entity(entity).insert(DestinationPoint(location));
                continue;
            }
        }

        if busy.timer.tick(time.delta()).just_finished() {
            let mut entity = commands.entity(entity);
            entity.remove::<(Busy, ActivityPlan)>();

            if let Ok(identifier) = q_activities.get_component::<Identifier>(plan.activity) {
                if identifier == &Identifier("Entrance".to_string()) {
                    entity.insert(ScheduledDespawn);
                }
            }
        }
    }
}

pub fn follow_routine(
    mut commands: Commands,
    mut q_people: Query<(Entity, &mut Routine), (Without<ActivityPlan>, Without<Reaction>)>,
) {
    for (entity, mut routine) in q_people.iter_mut() {
        let mut entity = commands.entity(entity);
        let total_activities = routine.activities.len();

        if total_activities > 0 {
            if let Some(item) = routine.next() {
                if let Some(activity) = item.activity {
                    entity.insert(ActivityPlan { activity });
                } else if let Some(search) = &item.search {
                    entity.insert(SearchingActivity(Identifier(search.to_string())));
                }
            } else {
                entity.remove::<Routine>();
            }
        }
    }
}
