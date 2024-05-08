use crate::score_system::time::TimeText;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use shared::Highscore;

#[derive(Component)]
pub struct HighscoreText {
    pub value: u64, //same datatype as secounds in Stopwatch::duration
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup code here
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Highscore: 0",
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
            left: Val::Px(20.0),
            ..default()
        }),
        HighscoreText { value: 0 },
    ));
}
pub fn update_highscore(
    mut query: Query<(&mut Text, &mut HighscoreText), With<HighscoreText>>,
    mut current_highscore: ResMut<crate::multiplayer_system::highscore::HighscoreResource>,
) {
    let (mut text, mut highscore_text) = query.single_mut();
    if current_highscore.0.time_in_seconds < highscore_text.value {
        highscore_text.value = current_highscore.0.time_in_seconds;
        text.sections[0].value = format!("Highscore: {}", highscore_text.value);
    }
}
