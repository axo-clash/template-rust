use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Action {
    #[serde(rename = "MOVE_LEFT")]
    MoveLeft,
    #[serde(rename = "MOVE_RIGHT")]
    MoveRight,
    #[serde(rename = "MOVE_UP")]
    MoveUp,
    #[serde(rename = "MOVE_DOWN")]
    MoveDown,
    #[serde(rename = "SHOOT_LEFT")]
    ShootLeft,
    #[serde(rename = "SHOOT_RIGHT")]
    ShootRight,
    #[serde(rename = "SHOOT_UP")]
    ShootUp,
    #[serde(rename = "SHOOT_DOWN")]
    ShootDown,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
