use super::{action_dto::ActionDTO, bot::BotDTO, case::CaseDTO};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDTO {
    pub id: i64,
    #[serde(rename = "firstBot")]
    pub first_bot: BotDTO,
    #[serde(rename = "secondBot")]
    pub second_bot: BotDTO,
    pub board: Vec<Vec<CaseDTO>>,
    #[serde(rename = "currentTurn")]
    pub current_turn: i32,
    pub actions: Vec<ActionDTO>,
}
