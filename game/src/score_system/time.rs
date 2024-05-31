use bevy::prelude::*;
use bevy::time::Stopwatch;
/// TimeText component
///
/// This component is used to store a `Stopwatch` that keeps track of the elapsed time.
#[derive(Component)]
pub struct TimeText {
    pub time: Stopwatch,
}
/// Sets up the initial state of the time display
///
/// This function is responsible for spawning the initial entities in the ECS for the time display.
/// It spawns a `TextBundle` entity with a `Text` component that displays the current elapsed time,
/// and a `TimeText` component that stores a `Stopwatch` to keep track of the elapsed time.
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
/// Updates the time display
///
/// This function is responsible for updating the time display in the ECS.
/// It fetches the `TimeText` and `Text` components of the time display entity,
/// updates the `Stopwatch` in the `TimeText` component to reflect the elapsed time,
/// and updates the `Text` component to display the new elapsed time.
///
/// # Arguments
///
/// * `time` - A reference to the `Time` resource, which provides the current time and delta time.
/// * `query` - A `Query` that fetches the `TimeText` and `Text` components of the time display entity.
pub fn change_time_text(
    time: Res<Time>,
    mut query: Query<(&mut TimeText, &mut Text), With<TimeText>>,
) {
    for (mut time_text, mut text) in query.iter_mut() {
        time_text.time.tick(time.delta());
        text.sections[0].value = format!("{:?}", time_text.time.elapsed().as_secs());
    }
}
