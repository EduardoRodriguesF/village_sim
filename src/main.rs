use village_sim::prelude::*;
use village_sim::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
