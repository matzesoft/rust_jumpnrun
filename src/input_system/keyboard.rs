use super::input_handler;
use bevy::prelude::{ButtonInput, DetectChanges, KeyCode, Res};
mod button_definitions {
    use bevy::prelude::KeyCode;

    //defines the different buttons used as well as their usage
    pub const JUMP_BUTTON: &KeyCode = &KeyCode::Space;
}
pub fn keyboard_input(keyboard_input: ButtonInput<KeyCode>) {
    for ev in keyboard_input.get_pressed() {
        match ev {
            button_definitions::JUMP_BUTTON => {
                //jump key pressed
                input_handler::jump_pressed();
            }
            _ => {}
        }
    }
}
