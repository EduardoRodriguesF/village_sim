use crate::prelude::*;

const POSITION_OFFSET_MULTIPLIER: f32 = 300.;

pub fn setup(mut commands: Commands) {
    for _ in 0..20 {
        let r: f32 = rand::random();
        let g: f32 = rand::random();
        let b: f32 = rand::random();
        let x_pos = rand::random::<f32>() * POSITION_OFFSET_MULTIPLIER;
        let y_pos = rand::random::<f32>() * POSITION_OFFSET_MULTIPLIER;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(r, g, b),
                    rect: Some(Rect::new(0., 0., 16., 32.)),
                    ..default()
                },
                ..default()
            },
            HeadlessTransform(Transform::from_xyz(x_pos, y_pos, 1.)),
        ));
    }
}
