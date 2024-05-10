use serde::{Deserialize, Serialize};

/// Stores the velocity and the translation of a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMovement {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub translation_x: f32,
    pub translation_y: f32,
}

/// Stores the name and movement of a player. Needed to show the
/// position of other players in the background of the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMovedUpdate {
    pub id: u64,
    pub movement: PlayerMovement,
}

/// The current best time for the level. If `time_in_seconds` is zero
/// there is no highscore yet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highscore {
    pub time_in_seconds: u64,
}

/// Messages sent from the player to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerMessage {
    Ping,
    JoinGame(PlayerMovement),
    PlayerMoved(PlayerMovement),
    RequestPossibleHighscore(Highscore),
    LeaveGame,
}

/// Messages sent from the server to the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
    InformAboutHighscore(Highscore),
    UpdateMovedPlayers(Vec<PlayerMovedUpdate>),
}
