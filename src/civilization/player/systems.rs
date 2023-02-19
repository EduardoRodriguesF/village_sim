use super::prelude::*;
use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    q_entrances: Query<&HeadlessTransform, With<Entrance>>,
) {
    let entrance_pos = q_entrances.iter().next().expect("No entrances found.");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
                rect: Some(Rect::new(0., 0., 16., 32.)),
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
        *entrance_pos,
        Velocity::new(0., 0.),
        Collider::new(Vec2::new(6., 6.), bevy::sprite::Anchor::BottomCenter),
        PlayerTag,
    ));
}
