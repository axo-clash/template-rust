use super::{case_content::CaseContent, coord::CoordDTO, item::ItemDTO};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDTO {
    pub coordinates: CoordDTO,
    pub content: Option<CaseContent>,
    #[serde(default)]
    pub items: Option<Vec<ItemDTO>>,
}
