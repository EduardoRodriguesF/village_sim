use super::prelude::*;
use crate::activities::prelude::*;
use crate::movement::prelude::*;
use crate::cursor::Cursor;
use rand::prelude::*;
use bevy_prototype_debug_lines::*;

pub fn setup_tracking_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_section = move |color, value: &str| {
        TextSection::new(
            value,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 16.0,
                color,
            },
        )
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([text_section(
                Color::GREEN,
                "NPC Tracking",
            )]));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Position: "),
                    text_section(Color::CYAN, ""),
                ]),
                PositionText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Velocity: "),
                    text_section(Color::CYAN, ""),
                ]),
                VelocityText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Routine length: "),
                    text_section(Color::CYAN, ""),
                ]),
                RoutineLengthText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Routine current index: "),
                    text_section(Color::CYAN, ""),
                ]),
                RoutineCurrentIndexText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Routine is loop? "),
                    text_section(Color::CYAN, ""),
                ]),
                RoutineLoopText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Activity location: "),
                    text_section(Color::CYAN, ""),
                ]),
                ActivityLocationText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::GREEN, "Remaining busy time: "),
                    text_section(Color::CYAN, ""),
                ]),
                ActivityTimerText,
            ));
        });
}

pub fn attach_log(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    cursor: Res<Cursor>,
    q_npcs: Query<(Entity, &NpcStats, &Transform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = cursor.relative_position {
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
                for (entity, _, _) in q_npcs.iter() {
                    commands.entity(entity).remove::<DebugTracking>();
                }

                npcs_by_distance.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
                let (entity, stats, _distance) = npcs_by_distance[0];

                commands.entity(entity).insert(DebugTracking);

                info!("{:#?}", stats);
            }
        }
    }
}

pub fn create_npc(mut commands: Commands, buttons: Res<Input<MouseButton>>, mut seed: ResMut<Seed>, cursor: Res<Cursor>) {
    if buttons.pressed(MouseButton::Right) {
        if let Some(cursor_pos) = cursor.relative_position {
            let r: f32 = seed.rng.gen();
            let g: f32 = seed.rng.gen();
            let b: f32 = seed.rng.gen();

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(r, g, b),
                        rect: Some(Rect::new(0., 0., 16., 32.)),
                        anchor: bevy::sprite::Anchor::BottomCenter,
                        ..default()
                    },
                    ..default()
                },
                NpcStats {
                    speed: seed.rng.gen_range(1.0..1.7),
                },
                Routine {
                    activities: vec![
                        RoutineItem::from_search("MarketTent")
                    ],
                    is_loop: true,
                    ..default()
                },
                HeadlessTransform(Transform::from_xyz(cursor_pos.x, cursor_pos.y, 1.)),
                Velocity::new(0., 0.),
            ));
        }
    }
}

pub fn trace_path(mut lines: ResMut<DebugLines>, q_tracked: Query<(&HeadlessTransform, &InstructionsToDestination), With<DebugTracking>>) {
    if let Ok((transform, instructions)) = q_tracked.get_single() {
        let mut last_step = transform.translation;

        for node in instructions.iter() {
            let vec2_node = node_to_vec2(*node);
            let step = Vec3::new(vec2_node.x, vec2_node.y, 1.);

            lines.line(last_step, step, 0.);

            last_step = step;
        }
    }
}

pub fn update_position_text(
    q_tracked: Query<&HeadlessTransform, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<PositionText>>,
) {
    if let Ok(transform) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            let x = transform.translation.x;
            let y = transform.translation.y;

            text.sections[1].value = format!("({x:.2}, {y:.2})");
        }
    }
}

pub fn update_velocity_text(
    q_tracked: Query<&Velocity, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<VelocityText>>,
) {
    if let Ok(velocity) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            let x = velocity.x;
            let y = velocity.y;

            text.sections[1].value = format!("({x:.2}, {y:.2})");
        }
    }
}

pub fn update_routine_len_text(
    q_tracked: Query<&Routine, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<RoutineLengthText>>,
) {
    if let Ok(routine) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            text.sections[1].value = format!("{}", routine.activities.len());
        }
    }
}

pub fn update_routine_idx_text(
    q_tracked: Query<&Routine, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<RoutineCurrentIndexText>>,
) {
    if let Ok(routine) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            text.sections[1].value = format!("{:?}", routine.current_idx);
        }
    }
}

pub fn update_routine_loop_text(
    q_tracked: Query<&Routine, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<RoutineLoopText>>,
) {
    if let Ok(routine) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            text.sections[1].value = format!("{:?}", routine.is_loop);
        }
    }
}

pub fn update_activity_location_text(
    q_tracked: Query<&Busy, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<ActivityLocationText>>,
) {
    if let Ok(busy) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            if let Some(location) = busy.location {
                text.sections[1].value = format!("({}, {})", location.x, location.y);
            }
        }
    }
}

pub fn update_activity_timer_text(
    q_tracked: Query<&Busy, With<DebugTracking>>,
    mut q_text: Query<&mut Text, With<ActivityTimerText>>,
) {
    if let Ok(busy) = q_tracked.get_single() {
        if let Ok(mut text) = q_text.get_single_mut() {
            text.sections[1].value = format!("{:.2}s", busy.timer.remaining_secs());
        }
    }
}
