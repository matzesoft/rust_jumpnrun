use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AsserServer>) {
    let mut camera = Camera2dBundle::default();
    commands.spawn(camera);

    let ldtk_handle = asset_server.load("jump_n_run.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
} 
