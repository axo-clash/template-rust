use super::bot::BotDTO;
use super::game::GameDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameBotDTO {
    #[serde(default)]
    pub game: Option<GameDTO>,
    #[serde(default)]
    pub bot: Option<BotDTO>,
}
