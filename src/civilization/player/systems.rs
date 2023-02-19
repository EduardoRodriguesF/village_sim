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
                color: Color::BLACK,
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

pub fn movement_input(
    keys: Res<Input<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<PlayerTag>>,
) {
    if let Ok(mut vel) = q_player.get_single_mut() {
        if keys.pressed(KeyCode::D) {
            vel.x = 1.0;
        } else if keys.pressed(KeyCode::A) {
            vel.x = -1.0;
        } else {
            vel.x = 0.0;
        }

        if keys.pressed(KeyCode::W) {
            vel.y = 1.0;
        } else if keys.pressed(KeyCode::S) {
            vel.y = -1.0;
        } else {
            vel.y = 0.0;
        }

        if keys.pressed(KeyCode::LShift) {
            vel.x *= 2.;
            vel.y *= 2.;
        }
    }
}
