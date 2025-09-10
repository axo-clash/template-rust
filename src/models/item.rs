use super::{bot::BotDTO, item_type::ItemType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDTO {
    pub r#type: ItemType,
    pub owner: BotDTO,
}
