use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{ResMut, Resource},
    },
    prelude::{Deref, DerefMut},
};
use shared::Highscore;

/// Bevy resource wrapper for the highscore.
#[derive(Resource, Deref, DerefMut)]
pub struct HighscoreResource(pub Highscore);

/// Bevy event to be fired when server sends info about a new highscore.
#[derive(Event)]
pub struct HighscoreInfoEvent(pub Highscore);

/// Called when the server informs about a new highscore.
/// Updates the highscore resource with the new highscore.
pub fn highscore_updated(
    mut events: EventReader<HighscoreInfoEvent>,
    mut current_highscore: ResMut<HighscoreResource>,
) {
    for ev in events.read() {
        if ev.0.time_in_seconds == 0 {
            println!("No highscore yet. Start playing!");
        } else {
            current_highscore.0 = ev.0.to_owned();
            println!(
                "Current highscore: {} seconds from player {}.",
                current_highscore.time_in_seconds, current_highscore.player_name
            );
        }
    }
}
