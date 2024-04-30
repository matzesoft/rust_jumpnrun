use crate::asset_system::ghost_physics::GhostColliderBundle;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Transform, *};
use bevy::sprite::SpriteSheetBundle;
use bevy::utils::hashbrown::HashMap;
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
use bevy_ecs_ldtk::prelude::*;
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode,
    connection::{ConnectionConfiguration, ConnectionEvent, ConnectionLostEvent},
    Client, QuinnetClientPlugin,
};
use bevy_rapier2d::prelude::*;

use bevy_rapier2d::dynamics::Velocity;
use std::{thread::sleep, time::Duration};

use crate::asset_system::players::{GhostPlayer, GhostPlayerBundle, Player};

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
            update_player_movement.run_if(is_player_connected),
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
    println!("Connecting to server...");
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    while let Some(message) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::Pong => println!("Received pong 🏓"),
            ServerMessage::UpdateMovedPlayers(players_moved_updates) => {
                let mut player_id_list: Vec<u64> = Vec::new();
                let mut player_velocities_server: HashMap<u64, Vec2> = HashMap::new();
                let mut player_transforms_server: HashMap<u64, Vec2> = HashMap::new();
                for update in players_moved_updates.iter() {
                    let movement = &update.movement;

                    player_id_list.push(update.id);

                    player_velocities_server.insert(
                        update.id,
                        Vec2::new(movement.velocity_x, movement.velocity_y),
                    );

                    player_transforms_server.insert(
                        update.id,
                        Vec2::new(movement.translation_x, movement.translation_y),
                    );
                }
                println!("Received player updates: {:?}", player_id_list);
                for (
                    mut ghost_velocity,
                    mut ghost_transform,
                    mut transform,
                    mut ghostPlayer,
                    entity,
                ) in &mut query
                {
                    if player_id_list.contains(&ghostPlayer.id) {
                        println!("Updating player with id: {}", ghostPlayer.id);
                        let server_velocity =
                            player_velocities_server.get(&ghostPlayer.id).unwrap();
                        let server_transform =
                            player_transforms_server.get(&ghostPlayer.id).unwrap();
                        ghost_velocity.linvel.x = server_velocity.x;
                        ghost_velocity.linvel.y = server_velocity.y;
                        transform.translation =
                            Vec3::new(server_transform.x, server_transform.y, 0.0);

                        //remove the player from the list
                        player_id_list.retain(|&x| x != ghostPlayer.id);
                    } else {
                        println!("Despawning player with id: {}", ghostPlayer.id);
                        //remove the player
                        commands.entity(entity).despawn();
                        commands.entity(entity).remove::<GhostPlayer>();
                    }
                }
                for id in player_id_list {
                    //spawn the player
                    println!("Spawning player with id: {}", id);

                    let texture_handle = asset_server.load("player_sprites/test1.png");
                    println!("loading test.png");
                    let texture_atlas = TextureAtlas::from_grid(
                        texture_handle.clone(),
                        Vec2::new(16.0, 16.0),
                        1,
                        1,
                        None,
                        None,
                    );

                    let texture_atlas_handle = asset_server.add(texture_atlas.clone());
                    commands.spawn(GhostPlayerBundle {
                        ghost_player: GhostPlayer { id },
                        sprite_sheet_bundle: SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: Transform {
                                translation: Vec3::new(300.0, 300.0, 0.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ghost_collider_bundle: GhostColliderBundle {
                            collider: Collider::cuboid(8.0, 8.0),
                            rigid_body: RigidBody::Dynamic,
                            friction: Friction::new(0.0),
                            rotation_constraints: LockedAxes::ROTATION_LOCKED,
                            ..Default::default()
                        },
                    });
                }
            }
        }
    }
}

pub fn update_player_movement(
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
