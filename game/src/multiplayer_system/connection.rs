use std::{thread::sleep, time::Duration};

use bevy::{
    app::{App, AppExit, Startup, Update},
    ecs::{
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, Res, ResMut},
    },
};
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode,
    connection::{ConnectionConfiguration, ConnectionEvent, ConnectionLostEvent},
    Client, QuinnetClientPlugin,
};

use crate::asset_system::players::Player;
use crate::input_system::input_handler::InputHandler;

use shared::{PlayerMessage, ServerMessage};

pub fn setup_client(app: &mut App) {
    app.add_plugins(QuinnetClientPlugin::default());
    app.add_systems(Startup, start_connection);
    app.add_systems(
        Update,
        (
            handle_connection_event,
            handle_connection_lost_event,
            handle_server_messages.run_if(is_player_connected),
            player_moved,
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

fn handle_connection_event(mut connection_event: EventReader<ConnectionEvent>) {
    if !connection_event.is_empty() {
        println!("Player connected to server :)");
        connection_event.clear();
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
            .send_message(PlayerMessage::Disconnect {})
            .unwrap();

        // TODO: event to let the async client send his last messages.
        sleep(Duration::from_secs_f32(0.1));
    }
}

fn handle_server_messages(mut client: ResMut<Client>) {
    while let Some(message) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::Pong => println!("Received pong 🏓"),
        }
    }
}

pub fn player_moved(mut client: ResMut<Client>, mut query: Query<&mut InputHandler, With<Player>>) {
    for mut input_handler in &mut query {
        if input_handler.walking != 0.0 {
            client.connection().try_send_message(PlayerMessage::PlayerWalked { direction: input_handler.walking });
        }
    }
}
