use super::{bot::BotDTO, game::GameDTO};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameBotDTO {
    pub game: GameDTO,
    pub bot: BotDTO,
}
