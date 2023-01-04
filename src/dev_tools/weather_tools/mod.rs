use crate::prelude::*;

pub struct WeatherToolsPlugin;

impl Plugin for WeatherToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(notify_weather_change);
    }
}

fn notify_weather_change(weather: Res<Weather>) {
    if weather.is_changed() {
        info!("Weather change: {:?}", weather.into_inner())
    }
}
