use super::bot::BotDTO;
use super::item_type::ItemType;
use super::serde_utils::lenient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDTO {
    #[serde(rename = "type", default, deserialize_with = "lenient")]
    pub r#type: Option<ItemType>,
    #[serde(default)]
    pub owner: Option<BotDTO>,
}
