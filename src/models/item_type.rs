use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "BULLET_LEFT")]
    BulletLeft,
    #[serde(rename = "BULLET_RIGHT")]
    BulletRight,
    #[serde(rename = "BULLET_UP")]
    BulletUp,
    #[serde(rename = "BULLET_DOWN")]
    BulletDown,
}
