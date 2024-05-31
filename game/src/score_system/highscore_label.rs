use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::multiplayer_system::highscore::HighscoreInfoEvent;
/// This component is used to store the current highscore value.
#[derive(Component)]
pub struct HighscoreText {
    pub value: u64, //same datatype as seconds in Stopwatch::duration
}
/// Sets up the initial state of the highscore display
///
/// This function is responsible for spawning the initial entities in the ECS for the highscore display.
/// It spawns a `TextBundle` entity with a `Text` component that displays the current highscore,
/// and a `HighscoreText` component that stores the current highscore value.
///
/// # Arguments
///
/// * `commands` - A mutable reference to the `Commands` struct, which is used to spawn entities and insert components in the ECS.
/// * `asset_server` - A reference to the `AssetServer`, which is used to load assets.
///
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup code here
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "No highscore yet!",
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
/// Updates the highscore display
///
/// This function is responsible for updating the highscore display in the ECS.
/// It listens for `HighscoreInfoEvent` events, and when one is received, it updates the `HighscoreText` component
/// and the `Text` component of the highscore display entity to reflect the new highscore.
///
/// # Arguments
///
/// * `events` - An `EventReader` for `HighscoreInfoEvent` events.
/// * `query` - A `Query` that fetches the `Text` and `HighscoreText` components of the highscore display entity.
pub fn update_highscore(
    mut events: EventReader<HighscoreInfoEvent>,
    mut query: Query<(&mut Text, &mut HighscoreText), With<HighscoreText>>,
) {
    for ev in events.read() {
        let (mut text, mut highscore_text) = query.single_mut();

        if ev.0.time_in_seconds != 0 {
            highscore_text.value = ev.0.time_in_seconds;
            text.sections[0].value = format!("Highscore: {}", highscore_text.value);
        }
    }
}
