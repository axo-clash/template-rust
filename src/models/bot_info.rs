use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotInfoDTO {
    pub name: String,
    pub version: String,
}
