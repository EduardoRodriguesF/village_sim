use bevy::prelude::*;

pub struct TimeManipulationPlugin;

impl Plugin for TimeManipulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(control_relative_time);
    }
}

fn control_relative_time(mut time: ResMut<Time>, keys: Res<Input<KeyCode>>) {
    let mut new_relative = None;
    let mut current = time.relative_speed();
    let possible_keys = vec![KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3];

    for (idx, key) in possible_keys.iter().enumerate() {
        if keys.just_pressed(*key) {
            current = idx as f32;
            new_relative = Some(current);
        }
    }

    if keys.just_pressed(KeyCode::Period) {
        new_relative = Some(current + 0.1);
    } else if keys.just_pressed(KeyCode::Comma) {
        new_relative = Some((current - 0.1).max(0.));
    }

    if let Some(value) = new_relative {
        time.set_relative_speed(value);
    }
}
