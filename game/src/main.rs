use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod asset_system;
mod input_system;
mod movement_system;
mod multiplayer_system;
mod score_system;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );
    app.add_plugins((
        LdtkPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
    ));

    multiplayer_system::connection::setup_client(&mut app);

    app.insert_resource(RapierConfiguration {
        gravity: Vec2::new(0.0, -9.81 * 50.0),
        ..Default::default()
    });
    app.insert_resource(LevelSelection::index(0));
    app.insert_resource(LdtkSettings {
        level_background: LevelBackground::Nonexistent,
        ..Default::default()
    });
    app.add_systems(
        Startup,
        (
            asset_system::assets_loading::setup,
            score_system::time::setup,
            score_system::highscore_label::setup,
        ),
    );

    app.add_systems(
        Update,
        (
            input_system::gamepad::gamepad_connections,
            input_system::gamepad::gamepad_input,
            input_system::keyboard::keyboard_input,
            movement_system::player_movement::player_movement,
            asset_system::walls::spawn_wall_collision,
            asset_system::walls::spawn_ground_sensor,
            asset_system::walls::ground_detection,
            asset_system::walls::update_on_ground,
            asset_system::traps::spawn_trap_collision,
            asset_system::traps::spawn_trap_sensor,
            asset_system::traps::trap_detection,
            asset_system::traps::update_on_trap,
            asset_system::finish_lines::spawn_finishline_collision,
            asset_system::finish_lines::spawn_finishline_sensor,
            asset_system::finish_lines::finishline_detection,
            asset_system::finish_lines::update_on_finishline,
            score_system::time::change_time_text,
            score_system::highscore_label::update_highscore,
        ),
    );
    app.register_ldtk_entity::<asset_system::players::PlayerBundle>("Player");
    app.register_ldtk_int_cell_for_layer::<asset_system::walls::WallBundle>("Map_IntGrid",1);
    app.register_ldtk_int_cell_for_layer::<asset_system::traps::TrapBundle>("Traps_IntGrid", 1);
    app.register_ldtk_int_cell_for_layer::<asset_system::finish_lines::FinishLineBundle>("Finish_Line_IntGrid", 1);

    app.add_event::<asset_system::finish_lines::FinishLineEvent>();

    app.run();
}
