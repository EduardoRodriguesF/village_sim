use super::prelude::*;

pub fn attach_log(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_npcs: Query<(Entity, &NpcStats, &Transform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = windows.get_primary().unwrap().cursor_position() {
            let mut npcs_by_distance: Vec<(Entity, &NpcStats, f32)> = q_npcs
                .iter()
                .filter_map(|(entity, stats, transform)| {
                    let distance = cursor_pos
                        .distance(Vec2::new(transform.translation.x, transform.translation.y));

                    if distance < 32. {
                        return Some((entity, stats, distance));
                    }

                    None
                })
                .collect();

            if !npcs_by_distance.is_empty() {
                npcs_by_distance.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
                let (entity, stats, _distance) = npcs_by_distance[0];

                commands.entity(entity).insert(DebugTracking);

                info!("{:#?}", stats);
            }
        }
    }
}
