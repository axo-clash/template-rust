use super::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayResponseDTO {
    pub action: Action,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<super::coord::CoordDTO>,
}
