use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseContent {
    #[serde(rename = "FIRST_BOT")]
    FirstBot,
    #[serde(rename = "SECOND_BOT")]
    SecondBot,
    #[serde(rename = "BULLET_LEFT")]
    BulletLeft,
    #[serde(rename = "BULLET_RIGHT")]
    BulletRight,
    #[serde(rename = "BULLET_UP")]
    BulletUp,
    #[serde(rename = "BULLET_DOWN")]
    BulletDown,
}
