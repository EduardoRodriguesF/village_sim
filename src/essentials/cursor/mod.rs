pub mod prelude;

use crate::prelude::*;
use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct Cursor {
    pub position: Option<Vec2>,
    pub relative_position: Option<Vec2>,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_system_to_stage(CoreStage::First, update_cursor_position);
    }
}

fn update_cursor_position(
    mut cursor: ResMut<Cursor>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    if let Some(cursor_pos) = windows.get_primary().unwrap().cursor_position() {
        cursor.position = Some(cursor_pos);

        cursor.relative_position = match q_camera.get_single() {
            Ok(camera_transform) => {
                let camera_pos = camera_transform.translation.truncate();
                let screen = Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

                Some(cursor_pos + camera_pos - screen / 2.)
            }
            Err(..) => None,
        };
    }
}
