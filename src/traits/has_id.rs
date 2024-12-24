use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait HasId {
    type Id: Hash + Copy + Eq + Serialize + for<'de> Deserialize<'de> + Ord;
    fn id(&self) -> Self::Id;
    fn with_id(self, id: Self::Id) -> Self;
}