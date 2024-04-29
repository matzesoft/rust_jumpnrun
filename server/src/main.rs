use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::TimePlugin};
use bevy_quinnet::server::{
    certificate::CertificateRetrievalMode, QuinnetServerPlugin, Server, ServerConfiguration,
};
use shared::{PlayerMessage, PlayerMovedUpdate, PlayerMovement, ServerMessage};

/// Represents a client currently playing the game. This component will be spawned
/// when the client calls the ``PlayerMessage::JoinGame`` function.
#[derive(Component)]
struct Player {
    /// The id given to the client from the ``bevy_quinnet`` library.
    client_id: u64,
}

/// Represents the velocity of a player in the game.
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

/// Represents the translation of a player in the game.
#[derive(Component)]
struct Translation {
    x: f32,
    y: f32,
}

/// Individuell timer for each player to check when the last movement update
/// has happend. If there was no update for a period of time, the player
/// will be removed from the server.
#[derive(Component)]
struct InactiveTimer(Timer);

/// Timer for sending updates to clients about the positons of the other players.
#[derive(Resource, Deref, DerefMut)]
struct UpdateMovedPlayersTimer(Timer);

pub fn main() {
    let mut app = App::new();
    app.add_plugins((
        ScheduleRunnerPlugin::default(),
        LogPlugin::default(),
        TimePlugin::default(),
    ));
    app.add_plugins(QuinnetServerPlugin::default());
    app.add_systems(Startup, start_listening);
    app.add_systems(
        Update,
        (
            handle_player_messages,
            send_updates_to_players,
            remove_inactive_players,
        ),
    );
    app.insert_resource(UpdateMovedPlayersTimer(Timer::from_seconds(
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

// TODO: Break function into multiple events
fn handle_player_messages(
    mut commands: Commands,
    mut players: Query<
        (
            Entity,
            &Player,
            &mut Velocity,
            &mut Translation,
            &mut InactiveTimer,
        ),
        With<Player>,
    >,
    mut server: ResMut<Server>,
) {
    let mut endpoint = server.endpoint_mut();

    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<PlayerMessage>(client_id) {
            match message {
                PlayerMessage::Ping => {
                    let _ = endpoint.send_message(client_id, ServerMessage::Pong);
                }
                PlayerMessage::JoinGame(movement) => {
                    println!("Player {} joined the game.", client_id);
                    commands.spawn((
                        Player {
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
                        InactiveTimer(Timer::from_seconds(10.0, TimerMode::Once)),
                    ));
                }
                PlayerMessage::PlayerMoved(movement) => {
                    for (_entity, player, mut velocity, mut translation, mut inactive_timer) in
                        players.iter_mut()
                    {
                        if player.client_id == client_id {
                            velocity.x = movement.velocity_x;
                            velocity.y = movement.velocity_y;
                            translation.x = movement.translation_x;
                            translation.y = movement.translation_y;

                            inactive_timer.0.reset();
                            break;
                        }
                    }
                }
                PlayerMessage::LeaveGame => {
                    for (entity, player, mut _velocity, mut _translation, mut _inactive_timer) in
                        players.iter_mut()
                    {
                        if player.client_id == client_id {
                            println!("Player {} left the game.", player.client_id);
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

fn send_updates_to_players(
    time: Res<Time>,
    mut timer: ResMut<UpdateMovedPlayersTimer>,
    server: Res<Server>,
    players: Query<(&Player, &Velocity, &Translation), With<Player>>,
) {
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    };

    let endpoint = server.endpoint();

    for client_id in endpoint.clients() {
        let mut players_movements: Vec<PlayerMovedUpdate> = Vec::new();

        for (player, velocity, translation) in players.iter() {
            if player.client_id != client_id {
                let update = PlayerMovedUpdate {
                    id: player.client_id,
                    movement: PlayerMovement {
                        velocity_x: velocity.x,
                        velocity_y: velocity.y,
                        translation_x: translation.x,
                        translation_y: translation.y,
                    },
                };
                players_movements.push(update);
            }
        }

        endpoint.try_send_message(
            client_id,
            ServerMessage::UpdateMovedPlayers(players_movements),
        );
    }
}

/// Ticks the `InactiveTimer` from each player and checks if the timer finished.
/// If so the player entitiy will be removed and the client disconnected.
fn remove_inactive_players(
    time: Res<Time>,
    mut server: ResMut<Server>,
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &mut InactiveTimer), With<Player>>,
) {
    for (entity, player, mut inactive_timer) in players.iter_mut() {
        inactive_timer.0.tick(time.delta());

        if inactive_timer.0.finished() {
            println!("Removed player {} due to inactivity.", player.client_id);

            commands.entity(entity).despawn();
            server.endpoint_mut().try_disconnect_client(player.client_id);
        }
    }
}
