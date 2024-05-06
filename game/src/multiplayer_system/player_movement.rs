use bevy::{
    ecs::{
        query::With,
        system::{Query, Res, ResMut, Resource},
    },
    prelude::{Deref, DerefMut},
    time::{Time, Timer},
    transform::components::GlobalTransform,
};
use bevy_quinnet::client::Client;
use bevy_rapier2d::dynamics::Velocity;
use shared::{PlayerMessage, PlayerMovement};

use crate::asset_system::players::Player;

/// Timer for sending updates to the server about the movement of the player.
#[derive(Resource, Deref, DerefMut)]
pub struct UpdatePlayerMovementTimer(pub Timer);

/// Sends the current velocity and transformation of the player to the server.
///
/// To decrease server load the message is only sent when the [UpdatePlayerMovementTimer]
/// is finished. Check the [setup_client] function for the exact timer interval.
pub fn update_player_movement(
    time: Res<Time>,
    mut timer: ResMut<UpdatePlayerMovementTimer>,
    client: Res<Client>,
    query: Query<(&Velocity, &GlobalTransform), With<Player>>,
) {
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    };

    for (velocity, transform) in &query {
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
