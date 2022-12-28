use super::prelude::*;
use crate::headless_transform::components::*;
use crate::movement::prelude::*;

pub fn determine_instructions(
    mut commands: Commands,
    map: Res<Map>,
    query: Query<
        (Entity, &HeadlessTransform, &DestinationPoint),
        Without<InstructionsToDestination>,
    >,
) {
    for (entity, transform, destination) in query.iter() {
        let start = Vec2::new(transform.translation.x, transform.translation.y);

        if start.distance(destination.0) < 4. {
            commands.entity(entity).remove::<DestinationPoint>();
        }

        let maybe_path = map.find_path_by_vec2(start, destination.0);

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
            let current_node =
                vec2_to_node(&Vec2::new(transform.translation.x, transform.translation.y));

            if current_node != next_instruction {
                let next_instruction = node_to_vec2(next_instruction);
                let current_translation =
                    Vec2::new(transform.translation.x, transform.translation.y);

                let dir = Vec2::normalize(next_instruction - current_translation);
                entity.insert(MovementIntention::new(dir.x, dir.y));
            } else {
                instructions.remove(0);
            }
        } else {
            entity.remove::<InstructionsToDestination>();
            entity.insert(MovementIntention::zero());
        }
    }
}
