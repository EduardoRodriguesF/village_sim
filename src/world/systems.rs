use super::prelude::*;
use crate::activities::prelude::*;
use crate::movement::prelude::*;
use rand::prelude::*;

const POSITION_OFFSET_MULTIPLIER: f32 = 300.;

pub fn spawn_entities(mut commands: Commands, map: Res<Map>) {
    for entity in map.entities.iter() {
        let maybe_entity_commands = match entity.identifier.as_str() {
            "MarketTent" => Some(commands.spawn((
                Activity {
                    avg_time_in_seconds: 12.,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::LIME_GREEN,
                        rect: Some(Rect::new(0., 0., 32., 32.)),
                        anchor: bevy::sprite::Anchor::BottomCenter,
                        ..default()
                    },
                    ..default()
                },
            ))),
            "Entrance" => Some(commands.spawn(Activity::default())),
            _ => None,
        };

        if let Some(mut entity_commands) = maybe_entity_commands {
            let pos = entity.position.extend(1.);
            let transform = HeadlessTransform(Transform::from_translation(pos));

            entity_commands.insert((Identifier(entity.identifier.to_owned()), transform));
        }
    }
}

pub fn setup(mut commands: Commands, mut seed: ResMut<Seed>) {
    for _ in 0..20 {
        let r: f32 = seed.rng.gen();
        let g: f32 = seed.rng.gen();
        let b: f32 = seed.rng.gen();
        let x_pos = seed.rng.gen::<f32>() * POSITION_OFFSET_MULTIPLIER;
        let y_pos = seed.rng.gen::<f32>() * POSITION_OFFSET_MULTIPLIER;

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
                activities: vec![RoutineItem::from_search("MarketTent"), RoutineItem::exit()],
                is_loop: true,
                ..default()
            },
            HeadlessTransform(Transform::from_xyz(x_pos, y_pos, 1.)),
            Velocity::new(0., 0.),
            Collider::new(Vec2::new(6., 6.), bevy::sprite::Anchor::BottomCenter),
        ));
    }
}

pub fn create_walls(mut commands: Commands, map: Res<Map>) {
    for (row_idx, row) in map.data.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if tile.is_none() {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex("333").unwrap(),
                            rect: Some(Rect::new(0., 0., 16., 16.)),
                            ..default()
                        },
                        ..default()
                    },
                    HeadlessTransform(Transform::from_xyz(
                        col_idx as f32 * 16.,
                        row_idx as f32 * 16.,
                        1.,
                    )),
                ));
            }
        }
    }
}
