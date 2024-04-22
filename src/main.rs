use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod asset_system;
mod input_system;
mod movement_system;
mod score_system;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(LdtkPlugin)
        .add_systems(
            Startup,
            (
                asset_system::assets_loading::setup,
                score_system::time::setup,
            ),
        )
        .add_systems(
            Update,
            (
                input_system::gamepad::gamepad_connections,
                input_system::gamepad::gamepad_input,
                input_system::keyboard::keyboard_input,
                movement_system::player_movement::player_movement,
                score_system::time::change_time_text,
            ),
        )
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
