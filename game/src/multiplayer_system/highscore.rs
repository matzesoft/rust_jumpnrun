use bevy::ecs::{
    event::{Event, EventReader},
    system::Res,
};
use bevy_quinnet::client::Client;
use shared::{Highscore, PlayerMessage};

use crate::asset_system::finish_lines::FinishLineEvent;

/// Bevy event to be fired when server sends info about a new highscore.
#[derive(Event)]
pub struct HighscoreInfoEvent(pub Highscore);

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
