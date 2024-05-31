/*use super::bullets;
use super::player;
use super::sprite;*/

use crate::asset_system::players::Player;
use crate::asset_system::walls::GroundDetection;
use crate::input_system::input_handler::InputHandler;
use bevy::input::gamepad::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

mod button_definitions {
    use bevy::prelude::GamepadButtonType;

    /// Defines the jump button used in the gamepad input handling
    ///
    /// This module contains a constant that represents the jump button on a gamepad.
    /// The constant is of type `GamepadButtonType`, which is an enum provided by the `bevy` crate.
    ///
    /// The jump button is set to `GamepadButtonType::South`, which typically corresponds to the 'A' button on an Xbox controller or the 'X' button on a PlayStation controller.
    ///

    pub const JUMP_BUTTON: GamepadButtonType = GamepadButtonType::South;
}
/// Handles the connection and disconnection of gamepads
///
/// This function is responsible for processing gamepad connection events and updating the game state accordingly.
/// It reads the gamepad events and if a connection event is detected, it checks whether a gamepad is already connected.
/// If no gamepad is connected, it sets the newly connected gamepad as the active gamepad.
/// If a gamepad is disconnected, it checks whether it was the active gamepad and if so, it removes it.
///
/// # Arguments
///
/// * `commands` - A mutable reference to the `Commands` struct, which is used to insert or remove resources in the ECS.
/// * `my_gamepad` - An optional resource that represents the currently connected gamepad. If no gamepad is connected, this will be None.
/// * `gamepad_evr` - The event reader for the gamepad. This is used to read the gamepad events that occurred since the last frame.
///

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

/// Handles the gamepad input for the player
///
/// This function is responsible for processing the input from the gamepad and updating the player's state accordingly.
/// It reads the gamepad events and applies the corresponding actions to the player's input handler.
/// For example, if the jump button is pressed, it sets the jumping state of the player to true.
///
/// # Arguments
///
/// * `my_gamepad` - An optional resource that represents the connected gamepad. If no gamepad is connected, this will be None.
/// * `gamepad_evr` - The event reader for the gamepad. This is used to read the gamepad events that occurred since the last frame.
/// * `player` - A query that fetches the input handler and ground detection of the player. The input handler is used to update the player's state based on the gamepad input, and the ground detection is used to check if the player is on the ground before allowing them to jump.

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
        if let Ok((p_handler, p_ground_detection)) = player.get_single_mut() {
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
