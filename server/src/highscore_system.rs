use bevy::prelude::*;
use bevy_quinnet::server::Server;
use shared::{Highscore, ServerMessage};

//
// ------> Components <------ //
//

/// Bevy resource wrapper for the highscore.
#[derive(Resource, Deref, DerefMut)]
pub struct HighscoreResource(pub Highscore);

//
// ------> Events <------ //
//

/// Called when a player left the game.
#[derive(Event)]
pub struct RequestHighscoreEvent {
    pub client_id: u64,
    pub possible_highscore: Highscore,
}

//
// ------> Systems <------ //
//

pub fn on_request_highscore(
    mut events: EventReader<RequestHighscoreEvent>,
    mut highscore: ResMut<HighscoreResource>,
    server: Res<Server>
) {
    for ev in events.read() {
        println!(
            "New highscore request received: {} seconds from player {}.",
            ev.possible_highscore.time_in_seconds, ev.client_id
        );

        if highscore.0.time_in_seconds > ev.possible_highscore.time_in_seconds
            || highscore.0.time_in_seconds == 0
        {
            highscore.0 = ev.possible_highscore.to_owned();

            server.endpoint()
                .try_broadcast_message(ServerMessage::InformAboutHighscore(highscore.0.clone()));
        }
    }
}
