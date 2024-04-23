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
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -9.81 * 50.0),
            ..Default::default()
        })
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, asset_system::assets_loading::setup)

        .add_systems(
            Update,
            (
                input_system::gamepad::gamepad_connections,
                input_system::gamepad::gamepad_input,
                input_system::keyboard::keyboard_input,
                movement_system::player_movement::player_movement,
                asset_system::collision::spawn_wall_collision,
                asset_system::ground::spawn_ground_sensor,
                asset_system::ground::ground_detection,
                asset_system::ground::update_on_ground,
                score_system::time::change_time_text,
            ),
        )
        .register_ldtk_entity::<asset_system::players::PlayerBundle>("Player")
        .register_ldtk_int_cell::<asset_system::walls::WallBundle>(1)
        .run();
}
