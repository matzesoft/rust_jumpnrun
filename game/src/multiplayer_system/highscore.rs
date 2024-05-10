use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{Res, ResMut, Resource},
    },
    prelude::{Deref, DerefMut},
};
use bevy_quinnet::client::Client;
use shared::{Highscore, PlayerMessage};

use crate::asset_system::finish_lines::FinishLineEvent;

/// Bevy resource wrapper for the highscore.
#[derive(Resource, Deref, DerefMut)]
pub struct HighscoreResource(pub Highscore);

/// Bevy event to be fired when server sends info about a new highscore.
#[derive(Event)]
pub struct HighscoreInfoEvent(pub Highscore);

/// Called when the server informs about a new highscore.
/// Updates the highscore resource with the new highscore.
pub fn new_highscore_info_from_server(
    mut events: EventReader<HighscoreInfoEvent>,
    mut current_highscore: ResMut<HighscoreResource>,
) {
    for ev in events.read() {
        if ev.0.time_in_seconds == 0 {
            println!("No highscore yet. Start playing!");
        } else {
            current_highscore.0 = ev.0.to_owned();
            println!("Current highscore: {}", current_highscore.time_in_seconds);
        }
    }
}

/// Called when the player finishes the level. Sends a request to the server if the player has set a new highscore.
/// If yes the server sends a [`ServerMessage::InformAboutHighscore`] message.
pub fn on_player_finish_level(mut events: EventReader<FinishLineEvent>, client: Res<Client>) {
    for ev in events.read() {
        let highscore = Highscore {
            time_in_seconds: ev.elapsed_time,
        };
        client
            .connection()
            .try_send_message(PlayerMessage::RequestPossibleHighscore(highscore));
    }
}
