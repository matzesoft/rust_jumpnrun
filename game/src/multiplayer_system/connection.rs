use std::{thread::sleep, time::Duration};

use bevy::{
    app::{App, AppExit, Startup, Update},
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

use crate::asset_system::players::Player;

use shared::{PlayerMessage, PlayerMovement, ServerMessage};

pub fn setup_client(app: &mut App) {
    app.add_plugins(QuinnetClientPlugin::default());
    app.add_systems(Startup, start_connection);
    app.add_systems(
        Update,
        (
            handle_connection_event,
            handle_connection_lost_event,
            handle_server_messages.run_if(is_player_connected),
            updte_player_movement.run_if(is_player_connected),
            on_app_exit,
        ),
    );
}

fn start_connection(mut client: ResMut<Client>) {
    // TODO: Remove unwrap!

    client
        .open_connection(
            ConnectionConfiguration::from_strings("127.0.0.1:6000", "0.0.0.0:0").unwrap(),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}

fn handle_connection_event(
    client: Res<Client>,
    mut connection_event: EventReader<ConnectionEvent>,
) {
    if !connection_event.is_empty() {
        println!("Player connected to server :)");
        connection_event.clear();

        // TODO: Set Connect function at a better fitting app cycle point!
        let message = PlayerMessage::JoinGame {
            player_name: "Bobert".to_string(), // TODO: Set player name dynamically,
            movement: PlayerMovement {
                velocity_x: 0.0,
                velocity_y: 0.0,
                translation_x: 0.0,
                translation_y: 0.0,
            },
        };

        client.connection().try_send_message(message);
    }
}

fn is_player_connected(client: Res<Client>) -> bool {
    client.connection().is_connected()
}

fn handle_connection_lost_event(mut connection_lost_event: EventReader<ConnectionLostEvent>) {
    if !connection_lost_event.is_empty() {
        println!("Player lost connection to server :(");
        connection_lost_event.clear();
    }
}

pub fn on_app_exit(app_exit_events: EventReader<AppExit>, client: Res<Client>) {
    if !app_exit_events.is_empty() {
        client
            .connection()
            .try_send_message(PlayerMessage::LeaveGame);

            println!("Received app exit event");
        // TODO: event to let the async client send his last messages.
        sleep(Duration::from_secs_f32(10.0));
    }
}

fn handle_server_messages(mut client: ResMut<Client>) {
    while let Some(message) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::Pong => println!("Received pong 🏓"),
            ServerMessage::UpdateMovedPlayers(players_moved_updates) => {
                for update in players_moved_updates.iter() {
                    let movement = &update.movement;

                    println!("Player {} moved:", update.player_name);
                    println!(
                        "Velocity: x {}, y {}",
                        movement.velocity_x, movement.velocity_y
                    );
                    println!(
                        "Translation: x {}, y {}",
                        movement.translation_x, movement.translation_y
                    );
                }
            }

            _ => {
                println!("Received unknown server message.");
            }
        }
    }
}

pub fn updte_player_movement(
    client: Res<Client>,
    mut query: Query<(&mut Velocity, &mut GlobalTransform), With<Player>>,
) {
    for (velocity, transform) in &mut query {
        let movement = PlayerMovement {
            velocity_x: velocity.linvel.x,
            velocity_y: velocity.linvel.y,
            translation_x: transform.translation().x,
            translation_y: transform.translation().y,
        };

        client
            .connection()
            .try_send_message(PlayerMessage::PlayerMoved(movement));
    }
}
