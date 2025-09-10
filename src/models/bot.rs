use super::{bot_data::BotDataDTO, bot_type::BotType, coord::CoordDTO};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotDTO {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub url: String,
    pub data: BotDataDTO,
    #[serde(rename = "botType")]
    pub bot_type: BotType,
    pub coordinates: CoordDTO,
    pub faster: bool,
}
