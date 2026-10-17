use super::bot_state::BotState;
use super::serde_utils::lenient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotDataDTO {
    #[serde(default)]
    pub life: Option<i32>,
    #[serde(rename = "nextTrapTurn", default)]
    pub next_trap_turn: Option<i32>,
    #[serde(rename = "forbiddenActions", default)]
    pub forbidden_actions: Option<i32>,
    #[serde(default, deserialize_with = "lenient")]
    pub state: Option<BotState>,
    #[serde(rename = "lastActionTimeMs", default)]
    pub last_action_time_ms: Option<i64>,
}
