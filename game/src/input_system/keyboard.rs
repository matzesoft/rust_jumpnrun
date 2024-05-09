use crate::asset_system::walls::GroundDetection;
use crate::asset_system::players::Player;
use crate::input_system::input_handler::InputHandler;
use bevy::prelude::*;

mod button_definitions {
    use bevy::prelude::KeyCode;

    //defines the different buttons used as well as their usage
    pub const JUMP_BUTTON: &KeyCode = &KeyCode::Space;
    pub const LEFT_BUTTON: &KeyCode = &KeyCode::A;
    pub const RIGHT_BUTTON: &KeyCode = &KeyCode::D;
}
pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut InputHandler, &GroundDetection), With<Player>>,
) {
    let (mut handler, ground_detection) =
        if let Ok((mut p_handler, p_ground_detection)) = player.get_single_mut() {
            (p_handler, p_ground_detection)
        } else {
            return;
        };
    //movement_direction is used to determine the direction of the player,
    //to enable the player to press left and right and then don't move
    let mut movement_direction: f32 = 0.0;
    for ev in keyboard_input.get_pressed() {
        match ev {
            button_definitions::JUMP_BUTTON => {
                //jump key pressed
                if !handler.jumping_pressed && ground_detection.on_ground {
                    handler.jumping = true;
                    handler.jumping_pressed = true;
                }
            }
            button_definitions::LEFT_BUTTON => {
                //left key pressed
                movement_direction += -1.0;
            }
            button_definitions::RIGHT_BUTTON => {
                //right key pressed
                movement_direction += 1.0;
            }
            _ => {}
        }
    }
    for ev in keyboard_input.get_just_released() {
        match ev {
            button_definitions::JUMP_BUTTON => {
                //jump key released
                handler.jumping_pressed = false;
            }
            _ => {}
        }
    }
    //gets called when anything on the keyboard is pressed or released
    if keyboard_input.is_changed() {
        handler.walking = movement_direction;
    }
}
