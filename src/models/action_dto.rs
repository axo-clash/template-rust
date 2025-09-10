use super::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDTO {
    pub id: i64,
    #[serde(rename = "botId")]
    pub bot_id: i64,
    pub action: Action,
    pub forbidden: bool,
    #[serde(rename = "executionTimeMs")]
    pub execution_time_ms: i64,
    #[serde(rename = "createdDate")]
    pub created_date: f64,
}
