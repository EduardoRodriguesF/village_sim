use super::prelude::*;
use crate::headless_transform::components::*;
use crate::world::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([text_section(
                Color::GREEN,
                "NPC Counting",
            )]));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::CYAN, ""),
                    text_section(Color::GREEN, " visible"),
                ]),
                VisibleCountingText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    text_section(Color::CYAN, ""),
                    text_section(Color::GREEN, " total"),
                ]),
                TotalCountingText,
            ));
        });
}

pub fn count_npcs(mut count: ResMut<NpcCount>, q_npcs: Query<Option<&HeadlessTransform>, With<NpcStats>>) {
    count.reset();

    for maybe_transform in q_npcs.iter() {
        count.total += 1;

        if let Some(..) = maybe_transform {
            count.visible += 1;
        }
    }
}

pub fn update_visible_text(count: Res<NpcCount>, mut q_text: Query<&mut Text, With<VisibleCountingText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[0].value = count.visible.to_string();
    }
}

pub fn update_total_text(count: Res<NpcCount>, mut q_text: Query<&mut Text, With<TotalCountingText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[0].value = count.total.to_string();
    }
}
