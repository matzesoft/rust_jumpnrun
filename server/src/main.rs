use bevy::{
    app::{ScheduleRunnerPlugin, Startup, Update},
    log::LogPlugin,
    prelude::{App, ResMut},
};
use bevy_quinnet::server::{
    certificate::CertificateRetrievalMode, QuinnetServerPlugin, Server, ServerConfiguration,
};
use shared::{PlayerMessage, ServerMessage};

pub fn main() {
    let mut app = App::new();
    app.add_plugins((ScheduleRunnerPlugin::default(), LogPlugin::default()));
    app.add_plugins(QuinnetServerPlugin::default());
    app.add_systems(Startup, start_listening);
    app.add_systems(Update, handle_player_messages);
    app.run();
}

fn start_listening(mut server: ResMut<Server>) {
    // TODO: Remove unwraps!
    server
        .start_endpoint(
            ServerConfiguration::from_string("127.0.0.1:6000").unwrap(),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "127.0.0.1".to_string(),
            },
        )
        .unwrap();
}

fn handle_player_messages(mut server: ResMut<Server>) {
    let mut endpoint = server.endpoint_mut();

    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<PlayerMessage>(client_id) {
            match message {
                PlayerMessage::Ping => {
                    let _ = endpoint.send_message(client_id, ServerMessage::Pong);
                },
                PlayerMessage::PlayerWalked { direction } => {
                    println!("Played {} walked: {}", client_id, direction);
                },
                PlayerMessage::Disconnect => {
                    println!("Received disconnect from client with id {}!", client_id);
                }
            }
        }
    }
}
