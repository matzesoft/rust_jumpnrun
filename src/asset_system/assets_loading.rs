use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("jump_n_run.ldtk"),
        ..Default::default()
    });
} 

pub const GRID_SIZE: i32 = 16;
