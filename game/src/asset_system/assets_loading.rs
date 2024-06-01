use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Sets up the game by spawning the camera and the ldtk world
///
/// This function setsup the camera and the ldtk file in wahich the world is saved.
/// # Arguments
/// * `commands` - A mutable reference to the commands
/// * `asset_server` - A resource that loads the assets
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("jump_n_run.ldtk"),
        ..Default::default()
    });
}