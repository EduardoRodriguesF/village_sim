use bevy::diagnostic::*;
use bevy::prelude::*;

pub struct BenchmarkPlugin;

impl Plugin for BenchmarkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(fps_counter);
    }
}

#[derive(Component)]
struct StatsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_section = move |color, value: &str| {
        TextSection::new(
            value,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 16.0,
                color,
            },
        )
    };

    commands.spawn((
        TextBundle::from_sections([
            text_section(Color::GREEN, "\nFPS (raw): "),
            text_section(Color::CYAN, ""),
            text_section(Color::GREEN, "\nFPS (SMA): "),
            text_section(Color::CYAN, ""),
            text_section(Color::GREEN, "\nFPS (EMA): "),
            text_section(Color::CYAN, ""),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        StatsText,
    ));
}

fn fps_counter(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<StatsText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw) = fps.value() {
            text.sections[1].value = format!("{raw:.2}");
        }
        if let Some(sma) = fps.average() {
            text.sections[3].value = format!("{sma:.2}");
        }
        if let Some(ema) = fps.smoothed() {
            text.sections[5].value = format!("{ema:.2}");
        }
    };
}
