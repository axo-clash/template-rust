use super::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayResponseDTO {
    pub action: Action,
}
