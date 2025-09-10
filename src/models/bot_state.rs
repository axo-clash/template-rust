use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BotState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "PLAYING")]
    Playing,
    #[serde(rename = "WAITING_FOR_OPPONENT")]
    WaitingForOpponent,
}
