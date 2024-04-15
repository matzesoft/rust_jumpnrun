use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod asset_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
                     .set(WindowPlugin{
                            primary_window: Some(Window {
                                title: "jump_n_run".into(),
                                fit_canvas_to_parent: true,
                                ..Default::default()
                            }),
                            ..Default::default()
                                            
                        })
                     .set(ImagePlugin::default_nearest())
                     )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, asset_system::assets_loading::setup)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .register_ldtk_entity::<asset_system::players::PlayerBundle>("Player")
        .register_ldtk_int_cell::<asset_system::walls::WallBundle>(1)
        .init_resource::<asset_system::walls::LevelWalls>()
        .run();
}
