use bevy::prelude::{Commands, Entity, Transform, *};
use bevy::{
    app::{App, Startup, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, Res, ResMut},
    },
    transform::components::GlobalTransform,
};
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode,
    connection::{ConnectionConfiguration, ConnectionEvent, ConnectionLostEvent},
    Client, QuinnetClientPlugin,
};

use bevy_rapier2d::dynamics::Velocity;

use crate::asset_system::players::GhostPlayer;

use crate::multiplayer_system::ghost_player;
use crate::multiplayer_system::highscore;
use crate::multiplayer_system::player_movement;
use crate::multiplayer_system::highscore::HighscoreInfoEvent;
use shared::{Highscore, PlayerMessage, PlayerMovement, ServerMessage};

/// The ip adress of the server. Use `127.0.0.1` when running the server locally, otherwise replace it
/// with the ip of your hosted server.
const SERVER_IP_ADDR: &'static str = "127.0.0.1";

/// Port the client should connect to on the server.
const SERVER_PORT: u16 = 8123;

/// Local address and port to bind to. See [`std::net::SocketAddrV4`] for more information.
const LOCAL_BIND_ADDR: &'static str = "0.0.0.0:0";

/// Adds all necessary plugins, resources and systems to the app to use multiplayer functionality.
pub fn setup_client(app: &mut App) {
    app.add_plugins(QuinnetClientPlugin::default());

    app.add_event::<HighscoreInfoEvent>();

    app.insert_resource(player_movement::UpdatePlayerMovementTimer(
        Timer::from_seconds(0.02, TimerMode::Repeating),
    ));
    app.insert_resource(highscore::HighscoreResource(Highscore {
        player_name: "".to_string(),
        time_in_seconds: 0, // 0 means -> No highscore set yet.
    }));

    app.add_systems(Startup, start_connection);
    app.add_systems(
        Update,
        (
            handle_connection_event,
            handle_connection_lost_event,
            handle_server_messages.run_if(is_player_connected),
            player_movement::update_player_movement.run_if(is_player_connected),
            highscore::highscore_updated,
        ),
    );
}

/// Opens the connection to the server with the [SERVER_IP_ADDR] and [SERVER_PORT] using the `bevy_quinnet` library.
fn start_connection(mut client: ResMut<Client>) {
    let server_addr_str = format!("{}:{}", SERVER_IP_ADDR, SERVER_PORT);
    let connection_config_result =
        ConnectionConfiguration::from_strings(&server_addr_str, LOCAL_BIND_ADDR);

    match connection_config_result {
        Ok(connection_config) => {
            let open_connection_result = client.open_connection(
                connection_config,
                CertificateVerificationMode::SkipVerification,
            );

            match open_connection_result {
                Ok(_) => {}
                Err(error) => println!("Error opening connection to server: {}", error),
            }
        }
        Err(error) => println!("Error creating connection configuration: {}", error),
    }
}

/// Called when the player connects to the server.
///
/// This event does **not** mean the player already joined the game. It just means that the connection
/// to the server was successful. To join the game the player has to send a [PlayerMessage::JoinGame] message.
fn handle_connection_event(
    client: Res<Client>,
    mut connection_event: EventReader<ConnectionEvent>,
) {
    if !connection_event.is_empty() {
        connection_event.clear();

        let message = PlayerMessage::JoinGame(PlayerMovement {
            velocity_x: 0.0,
            velocity_y: 0.0,
            translation_x: 0.0,
            translation_y: 0.0,
        });

        client.connection().try_send_message(message);
    }
}

fn is_player_connected(client: Res<Client>) -> bool {
    client.connection().is_connected()
}

/// Called when the player loses the connection to the server.
fn handle_connection_lost_event(mut connection_lost_event: EventReader<ConnectionLostEvent>) {
    if !connection_lost_event.is_empty() {
        connection_lost_event.clear();
        // TODO: Despawn ghost players using this event
    }
}

/// Handles all messages sent from the server to the client. Check [shared::ServerMessage] for all possible messages.
///
/// Messages received are then handled by the responsible system:
/// * [ServerMessage::UpdateMovedPlayers] - Handled by [`ghost_player::moved_players_updated`]
/// * [ServerMessage::InformAboutHighscore] - Handled by [`highscore::highscore_updated`]
fn handle_server_messages(
    mut client: ResMut<Client>,
    mut query: Query<
        (
            &mut Velocity,
            &mut GlobalTransform,
            &mut Transform,
            &GhostPlayer,
            Entity,
        ),
        With<GhostPlayer>,
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut ev_highscore_info: EventWriter<HighscoreInfoEvent>,
) {
    while let Some(message) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::Pong => println!("Received pong 🏓"),
            ServerMessage::UpdateMovedPlayers(players_moved_updates) => {
                ghost_player::moved_players_updated(
                    &mut query,
                    &mut commands,
                    &asset_server,
                    players_moved_updates,
                );
            }
            ServerMessage::InformAboutHighscore(new_highscore) => {
                ev_highscore_info.send(HighscoreInfoEvent(new_highscore));
            }
        }
    }
}
