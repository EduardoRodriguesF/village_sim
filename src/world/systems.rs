use super::prelude::*;
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
            DestinationNode(MapNode(3, 3)),
            HeadlessTransform(Transform::from_xyz(x_pos, y_pos, 1.)),
            Velocity::new(0., 0.),
        ));
    }
}

#[cfg(debug_assertions)]
pub fn initial_debug(seed: Res<Seed>) {
    println!("Seed: {}", seed.key);
}
