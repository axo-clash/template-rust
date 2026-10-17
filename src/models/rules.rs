use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRulesDTO {
    pub starting_life: Option<i32>,
    pub response_timeout_ms: Option<i32>,
    pub max_turns: Option<i32>,
    pub max_forbidden_actions: Option<i32>,
}
