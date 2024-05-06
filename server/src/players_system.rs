use bevy::prelude::*;
use bevy_quinnet::server::Server;
use shared::{PlayerMovedUpdate, PlayerMovement, ServerMessage};

use crate::highscore_system::HighscoreResource;

//
// ------> Components <------ //
//      

/// Represents a client currently playing the game. This component will be spawned
/// when the client calls the `PlayerMessage::JoinGame` function.
#[derive(Component)]
pub struct Player {
    /// The id given to the client from the ``bevy_quinnet`` library.
    client_id: u64,
}

/// Represents the velocity of a player in the game.
#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}

/// Represents the translation of a player in the game.
#[derive(Component)]
pub struct Translation {
    x: f32,
    y: f32,
}

/// Individuell timer for each player to check when the last movement update
/// has happend. If there was no update for a period of time, the player
/// will be removed from the server.
#[derive(Component)]
pub struct InactiveTimer(Timer);

//
// ------> Resources <------ //
//

/// Timer for sending updates to clients about the positons of the other players.
#[derive(Resource, Deref, DerefMut)]
pub struct UpdateMovedPlayersTimer(pub Timer);

//
// ------> Events <------ //
//

/// Called when there joins a new player to the server.
#[derive(Event)]
pub struct PlayerJoinedEvent {
    pub client_id: u64,
    pub movement: PlayerMovement,
}

/// Called when a player sends an update about his movement.
#[derive(Event)]
pub struct PlayerMovedEvent {
    pub client_id: u64,
    pub movement: PlayerMovement,
}

/// Called when a player left the game.
#[derive(Event)]
pub struct PlayerLeftEvent {
    pub client_id: u64,
}

//
// ------> Systems <------ //
//

/// Called when a player joines the game. Creates a new player entity with the given start position/movement.
pub fn on_player_joined(
    mut events: EventReader<PlayerJoinedEvent>,
    mut commands: Commands,
    server: Res<Server>,
    highscore: Res<HighscoreResource>
) {
    for ev in events.read() {
        println!("Player {} joined the game.", ev.client_id);

        commands.spawn((
            Player {
                client_id: ev.client_id,
            },
            Velocity {
                x: ev.movement.velocity_x,
                y: ev.movement.velocity_y,
            },
            Translation {
                x: ev.movement.translation_x,
                y: ev.movement.translation_y,
            },
            InactiveTimer(Timer::from_seconds(10.0, TimerMode::Once)),
        ));

        // Sends info about the current highscore to the player
        server.endpoint().try_send_message(
            ev.client_id,
            ServerMessage::InformAboutHighscore(highscore.0.clone()),
        );
    }
}

/// Called when a player send a update about his movement. Updates the values in the entity.
/// Informing other clients about the updated movement happens in [`send_updates_to_players`].
pub fn on_player_moved(
    mut events: EventReader<PlayerMovedEvent>,
    mut players: Query<
        (&Player, &mut Velocity, &mut Translation, &mut InactiveTimer),
        With<Player>,
    >,
) {
    for ev in events.read() {
        for (player, mut velocity, mut translation, mut inactive_timer) in players.iter_mut() {
            if player.client_id == ev.client_id {
                velocity.x = ev.movement.velocity_x;
                velocity.y = ev.movement.velocity_y;
                translation.x = ev.movement.translation_x;
                translation.y = ev.movement.translation_y;

                inactive_timer.0.reset();
                break;
            }
        }
    }
}

/// Called when a player left the game. Removes the player entity.
pub fn on_player_left(
    mut events: EventReader<PlayerLeftEvent>,
    mut commands: Commands,
    mut players: Query<(Entity, &Player), With<Player>>,
) {
    for ev in events.read() {
        for (entity, player) in players.iter_mut() {
            if player.client_id == ev.client_id {
                println!("Player {} left the game.", player.client_id);
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Informs all players about the current movements of the other players. The updates are sent in a
/// defined interval via the [`UpdateMovedPlayersTimer`]. Each update sent to a client contains
/// all players movement excluded his own.
pub fn send_updates_to_players(
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

/// Ticks the [`InactiveTimer`] from each player and checks if the timer finished.
/// If so the player entitiy will be removed and the client disconnected.
pub fn remove_inactive_players(
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
            server
                .endpoint_mut()
                .try_disconnect_client(player.client_id);
        }
    }
}
