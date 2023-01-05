use super::prelude::*;
use crate::headless_transform::components::*;
use crate::movement::prelude::*;
use crate::world::prelude::*;

const DESTINATION_THRESHOLD: f32 = 4.;

pub fn check_path_conditions_change(
    mut commands: Commands,
    weather: Res<Weather>,
    q_npcs: Query<Entity, With<InstructionsToDestination>>,
) {
    if weather.is_changed() {
        for entity in q_npcs.iter() {
            commands.entity(entity).insert(ReconsiderPath);
        }
    }
}

pub fn reconsider_path(mut commands: Commands, q_npcs: Query<Entity, With<ReconsiderPath>>) {
    for entity in q_npcs.iter() {
        commands
            .entity(entity)
            .insert(MovementIntention::zero())
            .remove::<(InstructionsToDestination, ReconsiderPath)>();
    }
}

pub fn determine_instructions(
    mut commands: Commands,
    weather: Res<Weather>,
    seed: Res<Seed>,
    map: Res<Map>,
    query: Query<
        (Entity, &HeadlessTransform, &DestinationPoint, &NpcStats),
        Without<InstructionsToDestination>,
    >,
) {
    let map = map.into_inner();

    for (entity, transform, destination, stats) in query.iter() {
        let start = transform.translation.truncate();

        let maybe_path = Pathfinder::new()
            .with_map(map.clone())
            .with_rng(seed.rng.clone())
            .with_stats(*stats)
            .with_weather(*weather)
            .find_path_by_vec2(start, destination.0);

        if let Some((instructions, _cost)) = maybe_path {
            commands
                .entity(entity)
                .insert(InstructionsToDestination(instructions));
        }
    }
}

pub fn follow_instructions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut InstructionsToDestination, &HeadlessTransform)>,
) {
    for (entity, mut instructions, transform) in query.iter_mut() {
        let mut entity = commands.entity(entity);

        if instructions.len() > 0 {
            let next_instruction = instructions[0];
            let current_translation = transform.translation.truncate();

            if current_translation.distance(next_instruction) > DESTINATION_THRESHOLD {
                let dir = Vec2::normalize_or_zero(next_instruction - current_translation);
                entity.insert(MovementIntention::new(dir.x, dir.y));
            } else {
                instructions.remove(0);
            }
        } else {
            entity.remove::<(DestinationPoint, InstructionsToDestination)>();
            entity.insert(MovementIntention::zero());
        }
    }
}
