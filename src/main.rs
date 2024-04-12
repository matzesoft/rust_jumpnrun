use bevy::prelude::*;

mod input_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Update,
            (
                input_system::gamepad::gamepad_connections,
                input_system::gamepad::gamepad_input,
                //input_system::keyboard::keyboard_input,
            ),
        )
        .run();
}
