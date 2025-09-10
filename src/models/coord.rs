use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordDTO {
    pub x: i32,
    pub y: i32,
}
