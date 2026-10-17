use super::action_dto::ActionDTO;
use super::bot::BotDTO;
use super::case::CaseDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDTO {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub rules: Option<super::rules::GameRulesDTO>,
    #[serde(rename = "firstBot", default)]
    pub first_bot: Option<BotDTO>,
    #[serde(rename = "secondBot", default)]
    pub second_bot: Option<BotDTO>,
    #[serde(default)]
    pub board: Option<Vec<Vec<CaseDTO>>>,
    #[serde(rename = "currentTurn", default)]
    pub current_turn: Option<i32>,
    #[serde(default)]
    pub actions: Option<Vec<ActionDTO>>,
}
