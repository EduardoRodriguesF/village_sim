use crate::prelude::*;

pub struct WeatherToolsPlugin;

impl Plugin for WeatherToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(notify_weather_change)
            .add_system(manual_weather_change);
    }
}

fn notify_weather_change(weather: Res<Weather>) {
    if weather.is_changed() {
        info!("Weather change: {:?}", weather.into_inner())
    }
}

fn manual_weather_change(mut weather: ResMut<Weather>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::PageUp) {
        weather.intensify();
    } else if keys.just_pressed(KeyCode::PageDown) {
        weather.lessen();
    }
}
