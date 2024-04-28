use serde::{Deserialize, Serialize};

/// Messages sent from the player to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerMessage {
    Ping,
    /// Asks the server if the choosen player name is already used by another player.
    /// Necessary to make the player name unique and provide a real highscore table.
    AskIfPlayerNameIsAvailable {
        player_name: String,
    },
    Connect(PlayerMovement),
    PlayerMoved(PlayerMovement),
    Disconnect {
        player_name: String,
    },
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
    UpdateMovedPlayers(Vec<PlayerMovement>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMovement {
    pub player_name: String,

    pub velocity_x: f32,
    pub velocity_y: f32,
    pub translation_x: f32,
    pub translation_y: f32,
}
