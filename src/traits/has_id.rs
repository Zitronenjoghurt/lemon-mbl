use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait HasId {
    type Id: Hash + Copy + Eq + Serialize + for<'de> Deserialize<'de>;
    fn id(&self) -> Self::Id;
}