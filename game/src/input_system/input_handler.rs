use bevy::prelude::Component;

/// Handles the input of the player
///
/// This struct handles the input of the player, it stores the information of the player's input.
/// It is used by the gamepad and keyboard file to set the specific values.
/// Player_movement uses this struct to move the player based on the input.
/// # Fields
/// * `jumping` - A boolean that stores if the player is jumping
/// * `walking` - A float that stores the direction the player is walking in, -1.0 for left, 1.0 for right, 0.0 for no movement, and values in between for slower movement
#[derive(Clone, Default, Component)]
pub struct InputHandler {
    pub jumping: bool,
    pub jumping_pressed: bool, // used to prevent multiple jumps by holding the jump button
    pub walking: f32,
}
