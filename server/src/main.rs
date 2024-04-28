use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::TimePlugin,
};
use bevy_quinnet::server::{
    certificate::CertificateRetrievalMode, QuinnetServerPlugin, Server, ServerConfiguration,
};
use shared::{PlayerMessage, PlayerMovement, ServerMessage};

pub fn main() {
    let mut app = App::new();
    app.add_plugins((
        ScheduleRunnerPlugin::default(),
        LogPlugin::default(),
        TimePlugin::default(),
    ));
    app.add_plugins(QuinnetServerPlugin::default());
    app.add_systems(Startup, (start_listening));
    app.add_systems(Update, (handle_player_messages, send_updates_to_players));
    app.insert_resource(UpdatePlayersTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
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

fn handle_player_messages(
    mut commands: Commands,
    mut players: Query<(&Player, &mut Velocity, &mut Translation), With<Player>>,
    mut server: ResMut<Server>,
) {
    let mut endpoint = server.endpoint_mut();

    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<PlayerMessage>(client_id) {
            match message {
                PlayerMessage::Ping => {
                    let _ = endpoint.send_message(client_id, ServerMessage::Pong);
                }
                PlayerMessage::Connect(movement) => {
                    commands.spawn((
                        Player {
                            name: movement.player_name,
                            client_id,
                        },
                        Velocity {
                            x: movement.velocity_x,
                            y: movement.velocity_y,
                        },
                        Translation {
                            x: movement.translation_x,
                            y: movement.translation_y,
                        },
                    ));
                }
                PlayerMessage::PlayerMoved(movement) => {
                    for (player, mut velocity, mut translation) in players.iter_mut() {
                        if player.client_id == client_id {
                            velocity.x = movement.velocity_x;
                            velocity.y = movement.velocity_y;
                            translation.x = movement.translation_x;
                            translation.y = movement.translation_y;
                            break;
                        }
                    }
                }
                PlayerMessage::Disconnect { player_name } => {
                    println!("Received disconnect from client with id {}!", client_id);
                }
                _ => {
                    println!("Received unknown Player Message.")
                }
            }
        }
    }
}

fn send_updates_to_players(
    time: Res<Time>,
    mut timer: ResMut<UpdatePlayersTimer>,
    mut server: ResMut<Server>,
    players: Query<(&Player, &Velocity, &Translation), With<Player>>,
) {
    timer.tick(time.delta());

    if timer.finished() {
        let mut endpoint = server.endpoint_mut();

        for client_id in endpoint.clients() {
            let mut players_movements: Vec<PlayerMovement> = Vec::new();

            for (player, velocity, translation) in players.iter() {
                if player.client_id != client_id {
                    players_movements.push(PlayerMovement {
                        player_name: player.name.clone(),
                        velocity_x: velocity.x,
                        velocity_y: velocity.y,
                        translation_x: translation.x,
                        translation_y: translation.y,
                    });
                }
            }

            endpoint.try_send_message(
                client_id,
                ServerMessage::UpdateMovedPlayers(players_movements),
            );
        }
    }
}

#[derive(Component)]
struct Player {
    name: String,
    client_id: u64,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Translation {
    x: f32,
    y: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct UpdatePlayersTimer(Timer);
