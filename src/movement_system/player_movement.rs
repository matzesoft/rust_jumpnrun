use crate::asset_system::players::*;
use crate::input_system::input_handler::InputHandler;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

///Defines the speed of the player
const PLAYER_SPEED: f32 = 300.0;
/// Moves the player based on the input
///
/// This function moves the player based on the input_handler.
/// # Arguments
/// * `query` - Query that fetches the velocity and input handler of the player, gets provided when called as a system

pub fn player_movement(mut query: Query<(&mut Velocity, &mut InputHandler), With<Player>>) {
    for (mut velocity, mut input_handler) in &mut query {
        //implements walking
        velocity.linvel.x = PLAYER_SPEED * input_handler.walking;
        //implements jumping
        if input_handler.jumping {
            velocity.linvel.y = 200.0;
            input_handler.jumping = false;
        }
    }
}
