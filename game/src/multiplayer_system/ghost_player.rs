use crate::asset_system::ghost_physics::GhostColliderBundle;
use crate::asset_system::players::{GhostPlayer, GhostPlayerBundle};
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    Commands, Entity, GlobalTransform, Mut, Query, Res, SpriteSheetBundle, TextureAtlas,
    TextureAtlasSprite, Transform, With,
};
use bevy::utils::hashbrown::HashMap;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Friction};
use shared::PlayerMovedUpdate;

pub fn moved_players_updated(
    query: &mut Query<
        (
            &mut Velocity,
            &mut GlobalTransform,
            &mut Transform,
            &GhostPlayer,
            Entity,
        ),
        With<GhostPlayer>,
    >,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    players_moved_updates: Vec<PlayerMovedUpdate>,
) {
    let mut player_id_list: Vec<u64> = Vec::new();
    let mut player_velocities_server: HashMap<u64, Vec2> = HashMap::new();
    let mut player_transforms_server: HashMap<u64, Vec2> = HashMap::new();
    get_player_id_list(
        players_moved_updates,
        &mut player_id_list,
        &mut player_velocities_server,
        &mut player_transforms_server,
    );
    println!("Received player updates: {:?}", player_id_list);
    for (mut ghost_velocity, mut ghost_transform, mut transform, mut ghostPlayer, entity) in query {
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
            despawn_player(commands, entity);
        }
    }
    for id in player_id_list {
        //spawn the player

        spawn_player(commands, &asset_server, id);
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
    println!("Updating player with id: {}", ghost_player.id);
    let server_velocity = player_velocities_server.get(&ghost_player.id).unwrap();
    let server_transform = player_transforms_server.get(&ghost_player.id).unwrap();
    ghost_velocity.linvel.x = server_velocity.x;
    ghost_velocity.linvel.y = server_velocity.y;
    transform.translation = Vec3::new(server_transform.x, server_transform.y, 0.0);

    //remove the player from the list
    player_id_list.retain(|&x| x != ghost_player.id);
}

fn despawn_player(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn();
    commands.entity(entity).remove::<GhostPlayer>();
}

fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, id: u64) {
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
