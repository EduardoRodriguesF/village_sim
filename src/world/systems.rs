use crate::prelude::*;

pub fn setup(mut commands: Commands) {
    for _ in 0..20 {
        let r: f32 = rand::random();
        let g: f32 = rand::random();
        let b: f32 = rand::random();
        let x_offset: f32 = rand::random();
        let y_offset: f32 = rand::random();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(r, g, b),
                    rect: Some(Rect::new(0., 0., 16., 32.)),
                    ..default()
                },
                ..default()
            },
            HeadlessTransform(Transform::from_xyz(x_offset * 300., y_offset * 300., 1.)),
        ));
    }
}
