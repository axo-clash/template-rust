use serde::{Deserialize, Serialize};

// Les variantes reflètent le contrat JSON (`BULLET_LEFT`, ...), d'où le préfixe commun.
#[allow(clippy::enum_variant_names)]
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
