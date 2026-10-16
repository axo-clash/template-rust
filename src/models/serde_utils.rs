use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

/// Deserializer for enum-typed `Option` fields: keeps `None` for explicit JSON
/// `null` and falls back to `None` on unknown enum variants instead of
/// failing the whole request.
pub fn lenient<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(value).unwrap_or_default())
}
