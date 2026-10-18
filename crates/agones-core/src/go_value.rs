//! Go JSON null for a non-pointer value leaves its zero value on fresh decode.
use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_go_value<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
