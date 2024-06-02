use crate::asset_system::players::Player;

use bevy::prelude::*;
/// Handles the movement of the camera
///
/// This function is responsible for updating the position of the camera based on the player's position.
/// It queries the ECS for the player's transform and the camera's transform and 2D camera component.
/// The camera's x position is then set to the player's x position.
///
/// # Arguments
///
/// * `query` - A mutable reference to a `Query` that fetches the player's transform.
/// * `camera` - A mutable reference to a `Query` that fetches the camera's transform and 2D camera component.
///
pub fn camera_movement(
    mut query: Query<&mut Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &Camera2d), Without<Player>>,
) {

    for transform in &mut query {
        //implements walking
        for (mut campos, _cam2d) in &mut camera.iter_mut() {
            campos.translation.x = transform.translation.x; // + CAMERA_OFFSET_X;
        }
    }
}
