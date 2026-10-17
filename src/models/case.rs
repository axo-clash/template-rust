use super::case_content::CaseContent;
use super::coord::CoordDTO;
use super::item::ItemDTO;
use super::serde_utils::lenient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDTO {
    #[serde(default)]
    pub coordinates: Option<CoordDTO>,
    #[serde(default, deserialize_with = "lenient")]
    pub content: Option<CaseContent>,
    #[serde(default)]
    pub items: Option<Vec<ItemDTO>>,
    #[serde(default)]
    pub trap: Option<super::trap::TrapDTO>,
}
