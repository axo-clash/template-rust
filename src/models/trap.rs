use super::bot_type::BotType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrapDTO {
    pub placed_by: BotType,
    pub placed_at_turn: i32,
    pub armed_from_turn: i32,
    pub expires_after_turn: i32,
}
