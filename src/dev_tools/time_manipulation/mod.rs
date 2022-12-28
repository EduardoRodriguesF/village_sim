use bevy::prelude::*;

pub struct TimeManipulationPlugin;

impl Plugin for TimeManipulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(control_relative_time);
    }
}

fn control_relative_time(mut time: ResMut<Time>, keys: Res<Input<KeyCode>>) {
    let possible_keys = vec![KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3];

    for (idx, key) in possible_keys.iter().enumerate() {
        if keys.just_pressed(*key) {
            time.set_relative_speed(idx as f32);
        }
    }

    let current = time.relative_speed();
    if keys.just_pressed(KeyCode::Period) {
        time.set_relative_speed(current + 0.1);
    } else if keys.just_pressed(KeyCode::Comma) {
        time.set_relative_speed((current - 0.1).max(0.));
    }
}
