/*use super::bullets;
use super::player;
use super::sprite;*/

use super::input_handler;
use crate::asset_system::ground::GroundDetection;
use crate::asset_system::players::Player;
use crate::input_system::input_handler::InputHandler;
use bevy::input::gamepad::*;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

mod button_definitions {
    use bevy::prelude::GamepadButtonType;

    //defines the different buttons used as well as their usage
    pub const JUMP_BUTTON: GamepadButtonType = GamepadButtonType::South;
}
pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.read() {
        // the ID of the gamepad

        if let GamepadEvent::Connection(connection_event) = ev {
            let id = connection_event.gamepad;
            if let GamepadConnection::Connected(info) = &connection_event.connection {
                println!(
                    "New gamepad connected with ID: {:?}, name: {}",
                    id, info.name
                );

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(id));
                }
            } else {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}

/// Handles the gamepad input
/// # Arguments
/// * `my_gamepad` - The gamepad that is connected
/// * `gamepad_evr` - The event reader for the gamepad
/// * `player` - Query that fetches the input handler of the player, gets provided when called as a system
pub fn gamepad_input(
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut player: Query<(&mut InputHandler, &GroundDetection), With<Player>>,
) {
    let _gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };
    let (mut handler, ground_detection) =
        if let Ok((mut p_handler, p_ground_detection)) = player.get_single_mut() {
            (p_handler, p_ground_detection)
        } else {
            return;
        };
    const DEADZONE: f32 = 0.2;
    for ev in gamepad_evr.read() {
        match ev {
            GamepadEvent::Axis(axis_changed) => {
                match axis_changed.axis_type {
                    GamepadAxisType::LeftStickX => {
                        //add small deadzones
                        if f32::abs(axis_changed.value) > DEADZONE {
                            handler.walking = axis_changed.value;
                            println!("Joystick moved on X Axis");
                        } else {
                            //joystick position reset to zero
                            handler.walking = 0.0;
                        }
                    }
                    GamepadAxisType::LeftStickY => {
                        if f32::abs(axis_changed.value) > DEADZONE {
                            //joystick moved beyond Deadzone
                            println!("Joystick moved on Y Axis");
                        } else {
                            //joystick position reset to zero
                        }
                    }
                    _ => {}
                }
                // Right Stick moved (X)
            }
            GamepadEvent::Button(button) => {
                // buttons are also reported as analog, so use a threshold

                match button.button_type {
                    GamepadButtonType::RightTrigger2 => {
                        if button.value > 0.0 {
                            //Button pressed
                        } else {
                            //Button not pressed
                        }
                    }
                    button_definitions::JUMP_BUTTON => {
                        if button.value > 0.0 {
                            //Button pressed
                            if !handler.jumping_pressed && ground_detection.on_ground {
                                handler.jumping = true;
                                handler.jumping_pressed = true;
                            }
                        } else {
                            //Button not pressed
                            handler.jumping_pressed = false;
                        }
                    }
                    _ => {}
                }
            }
            _ => {} // don't care about other inputs
        }
    }
}
