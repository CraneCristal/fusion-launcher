pub mod persistence;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    id: String,
    metadata: GameMetadata
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameData {
    metadata: GameMetadata
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameMetadata {

}

impl From<GameData> for Game {
    fn from(game_data: GameData) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            metadata: game_data.metadata
        }
    }
}