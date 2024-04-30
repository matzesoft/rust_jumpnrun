use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::TimePlugin};
use bevy_quinnet::server::{
    certificate::CertificateRetrievalMode, QuinnetServerPlugin, Server, ServerConfiguration,
};
use highscore_system::{HighscoreResource, RequestHighscoreEvent};
use players_system::{
    PlayerJoinedEvent, PlayerLeftEvent, PlayerMovedEvent, UpdateMovedPlayersTimer,
};
use shared::{Highscore, PlayerMessage, ServerMessage};

mod highscore_system;
mod players_system;

static SERVER_IP_ADDR: &'static str = "127.0.0.1";
static SERVER_PORT: u16 = 6000;

/// Creates the bevy app for the server with all required plugins, events, systems and resources.
pub fn main() {
    let mut app = App::new();
    app.add_plugins((
        ScheduleRunnerPlugin::default(),
        LogPlugin::default(),
        TimePlugin::default(),
        QuinnetServerPlugin::default(),
    ));

    app.add_event::<PlayerJoinedEvent>();
    app.add_event::<PlayerMovedEvent>();
    app.add_event::<PlayerLeftEvent>();
    app.add_event::<RequestHighscoreEvent>();

    app.add_systems(Startup, start_listening);
    app.add_systems(
        Update,
        (
            handle_player_messages,
            players_system::on_player_joined,
            players_system::on_player_moved,
            players_system::on_player_left,
            players_system::send_updates_to_players,
            players_system::remove_inactive_players,
            highscore_system::on_request_highscore,
        ),
    );

    app.insert_resource(UpdateMovedPlayersTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
    app.insert_resource(HighscoreResource(Highscore {
        player_name: "".to_string(),
        time_in_seconds: 0, // 0 means -> No highscore set yet.
    }));

    app.run();
}

/// Starts the endpoint of the server via the ``bevy_quinnet`` library.
fn start_listening(mut server: ResMut<Server>) {
    let local_bind_addr_str = format!("{}:{}", SERVER_IP_ADDR, SERVER_PORT);
    let server_config_result = ServerConfiguration::from_string(&local_bind_addr_str);

    match server_config_result {
        Ok(config) => {
            let cert_mode = CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: SERVER_IP_ADDR.to_string(),
            };

            let start_endpoint_result = server.start_endpoint(config, cert_mode);
            match start_endpoint_result {
                Ok(_) => {},
                Err(error) => println!("Failed to start server endpoint: {}", error),
            }
        }
        Err(error) => println!("Failed to parse server adress: {}", error),
    }
}

/// Handles all messages sent from the clients to the server. Each messages creates a new event
/// which is than handled by the responsible system.
fn handle_player_messages(
    mut server: ResMut<Server>,

    mut ev_player_joined: EventWriter<PlayerJoinedEvent>,
    mut ev_player_moved: EventWriter<PlayerMovedEvent>,
    mut ev_player_left: EventWriter<PlayerLeftEvent>,
    mut ev_highscore_request: EventWriter<RequestHighscoreEvent>,
) {
    // This mutable is required due to the `endpoint.try_receive_message_from` function call.
    // Seems like a rust analyer mistake to state that mut is not required.
    #![allow(unused_mut)]
    let mut endpoint = server.endpoint_mut();

    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<PlayerMessage>(client_id) {
            match message {
                PlayerMessage::Ping => {
                    let _ = endpoint.send_message(client_id, ServerMessage::Pong);
                }
                PlayerMessage::JoinGame(movement) => {
                    ev_player_joined.send(PlayerJoinedEvent {
                        client_id,
                        movement,
                    });
                }
                PlayerMessage::PlayerMoved(movement) => {
                    ev_player_moved.send(PlayerMovedEvent {
                        client_id,
                        movement,
                    });
                }
                PlayerMessage::RequestPossibleHighscore(possible_highscore) => {
                    ev_highscore_request.send(RequestHighscoreEvent {
                        client_id,
                        possible_highscore,
                    });
                }
                PlayerMessage::LeaveGame => {
                    ev_player_left.send(PlayerLeftEvent { client_id });
                }
            }
        }
    }
}
