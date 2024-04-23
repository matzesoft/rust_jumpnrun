use bevy::{
    app::{App, Startup, Update},
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
};
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
    QuinnetClientPlugin,
};

use crate::asset_system::players::Player;
use crate::input_system::input_handler::InputHandler;

use shared::{PlayerMessage, ServerMessage};

pub fn setup_client(app: &mut App) {
    app.add_plugins(QuinnetClientPlugin::default());
    app.add_systems(Startup, start_connection);
    app.add_systems(Update, (handle_server_messages, player_moved));
}

fn start_connection(mut client: ResMut<Client>) {
    client
        .open_connection(
            ConnectionConfiguration::from_strings("127.0.0.1:6000", "0.0.0.0:0").unwrap(),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}

fn handle_server_messages(mut client: ResMut<Client>) {
    while let Some(message) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        println!("New message from server!");
    }
}

pub fn player_moved(mut client: ResMut<Client>, mut query: Query<&mut InputHandler, With<Player>>) {
    for (mut input_handler) in &mut query {
        if (input_handler.walking != 0.0) {
            client.connection().try_send_message(PlayerMessage::Ping);
        }
    }
}