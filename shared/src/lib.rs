use serde::{Deserialize, Serialize};

/// Stores the velocity and the translation of a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMovement {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub translation_x: f32,
    pub translation_y: f32,
}

/// Stores the name and movement of a single player. Needed to show
/// the position of other players in the background of the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMovedUpdate {
    pub player_name: String,
    pub movement: PlayerMovement,
}

/// Messages sent from the player to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerMessage {
    Ping,
    /// Asks the server if the choosen player name is already used by another player.
    /// Necessary to make the player name unique and provide a real highscore table.
    AskIfPlayerNameIsAvailable {
        player_name: String,
    },
    Connect {
        player_name: String,
        movement: PlayerMovement,
    },
    PlayerMoved(PlayerMovement),
    Disconnect,
}

/// Messages sent from the server to the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
    /// Response to the ``PlayerMessage::AskIfPlayerNameIsAvailable`` message.
    ReponseIfPlayerNameIsAvailable {
        requested_player_name: String,
        available: bool,
    },
    UpdateMovedPlayers(Vec<PlayerMovedUpdate>),
}
