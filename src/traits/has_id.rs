use std::hash::Hash;

pub trait HasId {
    type Id: Hash + Copy + Eq;
    fn id(&self) -> Self::Id;
}