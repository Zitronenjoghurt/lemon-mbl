use serde::de::DeserializeOwned;
use serde::{Deserializer, Serialize, Serializer};
use std::sync::RwLock;

pub fn serialize<S, T>(lock: &RwLock<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match lock.read() {
        Ok(guard) => guard.serialize(serializer),
        Err(_) => Err(serde::ser::Error::custom("RwLock was poisoned during serialization")),
    }
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<RwLock<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let inner = T::deserialize(deserializer)?;
    Ok(RwLock::new(inner))
}