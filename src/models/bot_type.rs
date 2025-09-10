use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BotType {
    #[serde(rename = "FIRST_BOT")]
    FirstBot,
    #[serde(rename = "SECOND_BOT")]
    SecondBot,
}
