use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct TimeText {
    pub time: Stopwatch,
}
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup code here
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "0",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/Pixelfont.ttf"),
                font_size: 50.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(20.0),
            ..default()
        }),
        TimeText {
            time: Stopwatch::new(),
        },
    ));
}

pub fn change_time_text(
    time: Res<Time>,
    mut query: Query<(&mut TimeText, &mut Text), With<TimeText>>,
) {
    for (mut time_text, mut text) in query.iter_mut() {
        time_text.time.tick(time.delta());
        text.sections[0].value = format!("{:?}", time_text.time.elapsed().as_secs());
    }
}
