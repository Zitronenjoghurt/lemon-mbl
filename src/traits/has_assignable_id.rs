use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait HasAssignableId {
    type Id: Hash + Copy + Eq + Serialize + for<'de> Deserialize<'de>;
    fn get_id(&self) -> Self::Id;
    fn set_id(&mut self, id: Self::Id);
}