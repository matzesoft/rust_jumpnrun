use crate::asset_system::players::Player;
use bevy::ecs::system::WithEntity;
use bevy::prelude::*;

pub fn camera_movement(
    mut query: Query<(&mut Transform), With<Player>>,
    mut camera: Query<(&mut Transform, &Camera2d), Without<Player>>,
) {
    const CAMERA_OFFSET_X: f32 = 1280.0 / 4.0;
    for (transform) in &mut query {
        //implements walking
        for (mut campos, cam2d) in &mut camera.iter_mut() {
            campos.translation.x = transform.translation.x; // + CAMERA_OFFSET_X;
        }
    }
}
