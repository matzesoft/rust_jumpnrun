use crate::asset_system::ghost_physics::GhostColliderBundle;
use crate::asset_system::players::{GhostPlayer, GhostPlayerBundle};
use bevy::asset::AssetServer;
use bevy::ecs::event::{Event, EventReader};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    Commands, Entity, GlobalTransform, Mut, Query, Res, SpriteSheetBundle, TextureAtlas,
    TextureAtlasSprite, Transform, With,
};
use bevy::utils::hashbrown::HashMap;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Friction};
use shared::PlayerMovedUpdate;

/// Bevy event to be fired when server sends info about positions of ghost players.
#[derive(Event)]
pub struct GhostPlayersMovedEvent(pub Vec<PlayerMovedUpdate>);

pub fn moved_players_updated(
    mut events: EventReader<GhostPlayersMovedEvent>,
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
) {
    for ev in events.read() {
        let players_moved_updates = ev.0.to_owned();

        let mut player_id_list: Vec<u64> = Vec::new();
        let mut player_velocities_server: HashMap<u64, Vec2> = HashMap::new();
        let mut player_transforms_server: HashMap<u64, Vec2> = HashMap::new();
        get_player_id_list(
            players_moved_updates,
            &mut player_id_list,
            &mut player_velocities_server,
            &mut player_transforms_server,
        );
        for (mut ghost_velocity, mut ghost_transform, mut transform, mut ghostPlayer, entity) in
            &mut query
        {
            if player_id_list.contains(&ghostPlayer.id) {
                move_player(
                    &mut player_id_list,
                    &mut player_velocities_server,
                    &mut player_transforms_server,
                    &mut ghost_velocity,
                    &mut transform,
                    &mut ghostPlayer,
                );
            } else {
                println!("Despawning player with id: {}", ghostPlayer.id);
                //remove the player
                despawn_player(&mut commands, entity);
            }
        }
        for id in player_id_list {
            //spawn the player

            spawn_player(&mut commands, &asset_server, id);
        }
    }
}

fn get_player_id_list(
    players_moved_updates: Vec<PlayerMovedUpdate>,
    player_id_list: &mut Vec<u64>,
    player_velocities_server: &mut HashMap<u64, Vec2>,
    player_transforms_server: &mut HashMap<u64, Vec2>,
) {
    for update in players_moved_updates.iter() {
        let movement = &update.movement;

        player_id_list.push(update.id);

        let velocity = Vec2::new(movement.velocity_x, movement.velocity_y);
        player_velocities_server.insert(update.id, velocity);

        let transform = Vec2::new(movement.translation_x, movement.translation_y);
        player_transforms_server.insert(update.id, transform);
    }
}

fn move_player(
    player_id_list: &mut Vec<u64>,
    player_velocities_server: &mut HashMap<u64, Vec2>,
    player_transforms_server: &mut HashMap<u64, Vec2>,
    ghost_velocity: &mut Mut<Velocity>,
    transform: &mut Mut<Transform>,
    mut ghost_player: &mut &GhostPlayer,
) {
    let server_velocity = player_velocities_server.get(&ghost_player.id).unwrap();
    let server_transform = player_transforms_server.get(&ghost_player.id).unwrap();
    ghost_velocity.linvel.x = server_velocity.x;
    ghost_velocity.linvel.y = server_velocity.y;
    transform.translation = Vec3::new(server_transform.x, server_transform.y, 0.0);

    //remove the player from the list
    player_id_list.retain(|&x| x != ghost_player.id);
}
/// Despawns a ghost player entity
///
/// This function is responsible for despawning a ghost player entity from the ECS.
/// It removes the `GhostPlayer` component from the entity and then despawns the entity itself.
///
/// # Arguments
///
/// * `commands` - A mutable reference to the `Commands` struct, which is used to despawn entities and remove components in the ECS.
/// * `entity` - The `Entity` struct representing the ghost player to be despawned.
///
pub fn despawn_player(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn();
    commands.entity(entity).remove::<GhostPlayer>();
}
/// Spawns a new ghost player entity
///
/// This function is responsible for spawning a new ghost player entity in the ECS.
/// It loads the sprite for the player based on their id, creates a texture atlas from the sprite,
/// and then spawns a new entity with the `GhostPlayerBundle`.
///
/// # Arguments
///
/// * `commands` - A mutable reference to the `Commands` struct, which is used to spawn entities and insert components in the ECS.
/// * `asset_server` - A reference to the `AssetServer`, which is used to load assets.
/// * `id` - The id of the player to spawn.
///
fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, id: u64) {
    println!("Spawning player with id: {}", id);

    let texture_handle = asset_server.load(get_sprite_filename(id));
    println!("loading test.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::new(16.0, 16.0),
        29,
        31,
        None,
        Some(Vec2::new(16.0, 16.0)),
    );

    let texture_atlas_handle = asset_server.add(texture_atlas);
    commands.spawn(GhostPlayerBundle {
        ghost_player: GhostPlayer { id },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: Vec3::new(300.0, 300.0, 99999999.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ghost_collider_bundle: GhostColliderBundle {
            collider: Collider::cuboid(6.5, 8.0),
            rigid_body: RigidBody::Dynamic,
            friction: Friction::new(0.0),
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            ..Default::default()
        },
    });
}
/// Returns the filename of the sprite for a given player id
///
/// This function is responsible for generating the filename of the sprite for a given player id.
/// It uses the player id to select one of the 19 available player sprites.
///
/// # Arguments
///
/// * `player_id` - The id of the player for which to generate the sprite filename.
///
/// # Returns
///
/// This function returns a `String` that represents the filename of the sprite for the given player id.
///
fn get_sprite_filename(player_id: u64) -> String {
    format!("player_sprites/Charakter{}.png", (player_id % 19) + 1) //there are 19 different player sprites available, the 20th player gets the same sprite as the first player
}
