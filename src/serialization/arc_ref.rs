use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

/// Enables serializing Arc<T> by storing only a key instead of the full data.
/// Useful when the actual data can be looked up from a cache or registry.
pub trait ArcRefFromKey: Sized {
    /// The type used as a key for lookup (e.g., ID, handle, etc.)
    type Key: Serialize + for<'de> Deserialize<'de>;

    /// Convert this type into its lookup key
    fn to_key(&self) -> Self::Key;

    /// Reconstruct an Arc<Self> from a key (e.g., by looking up in a cache)
    fn from_key(key: &Self::Key) -> Option<Arc<Self>>;
}

/// Serializes an Arc<T> by converting it to its key
pub fn serialize<S, T>(data: &Arc<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ArcRefFromKey,
{
    T::to_key(data).serialize(serializer)
}

/// Deserializes an Arc<T> by first deserializing its key and then looking up the data
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Arc<T>, D::Error>
where
    D: Deserializer<'de>,
    T: ArcRefFromKey,
{
    let key = T::Key::deserialize(deserializer)?;
    T::from_key(&key)
        .ok_or_else(|| serde::de::Error::custom("Failed to reconstruct from key"))
}