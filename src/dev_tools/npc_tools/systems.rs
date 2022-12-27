use super::prelude::*;

pub fn attach_log(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_npcs: Query<(&NpcStats, &Transform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = windows.get_primary().unwrap().cursor_position() {
            let mut npcs_by_distance: Vec<(&NpcStats, f32)> = q_npcs
                .iter()
                .filter_map(|(stats, transform)| {
                    let distance = cursor_pos
                        .distance(Vec2::new(transform.translation.x, transform.translation.y));

                    if distance < 32. {
                        return Some((stats, distance));
                    }

                    None
                })
                .collect();

            if !npcs_by_distance.is_empty() {
                npcs_by_distance.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
                let (stats, _distance) = npcs_by_distance[0];

                info!("{:#?}", stats);
            }
        }
    }
}
