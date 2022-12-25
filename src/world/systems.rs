use super::prelude::*;
use crate::activities::prelude::*;
use crate::destinations::prelude::*;
use crate::movement::prelude::*;
use crate::prelude::*;
use rand::prelude::*;

const POSITION_OFFSET_MULTIPLIER: f32 = 300.;

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
                activities: vec![
                    Busy::from_seconds(10., Some(Vec2::new(800., 300.))),
                    Busy::from_seconds(5., Some(Vec2::new(800., 100.))),
                ],
                is_loop: true,
                ..default()
            },
            HeadlessTransform(Transform::from_xyz(x_pos, y_pos, 1.)),
            Velocity::new(0., 0.),
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

#[cfg(debug_assertions)]
pub fn initial_debug(seed: Res<Seed>) {
    println!("Seed: {}", seed.key);
}
