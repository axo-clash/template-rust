use super::action::Action;
use super::serde_utils::lenient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDTO {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub coordinates: Option<super::coord::CoordDTO>,
    #[serde(rename = "botId", default)]
    pub bot_id: Option<i64>,
    #[serde(default, deserialize_with = "lenient")]
    pub action: Option<Action>,
    #[serde(default)]
    pub forbidden: Option<bool>,
    #[serde(rename = "executionTimeMs", default)]
    pub execution_time_ms: Option<i64>,
    #[serde(rename = "createdDate", default)]
    pub created_date: Option<String>,
}
