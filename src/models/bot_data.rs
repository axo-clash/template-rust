use super::bot_state::BotState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotDataDTO {
    pub life: i32,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: i32,
    pub state: BotState,
    #[serde(rename = "lastActionTimeMs")]
    pub last_action_time_ms: i64,
}
