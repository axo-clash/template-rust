use super::bot_data::BotDataDTO;
use super::bot_type::BotType;
use super::coord::CoordDTO;
use super::serde_utils::lenient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotDTO {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub data: Option<BotDataDTO>,
    #[serde(rename = "botType", default, deserialize_with = "lenient")]
    pub bot_type: Option<BotType>,
    #[serde(default)]
    pub coordinates: Option<CoordDTO>,
    #[serde(default)]
    pub faster: Option<bool>,
}
