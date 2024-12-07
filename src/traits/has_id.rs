use std::hash::Hash;

pub trait HasId {
    type Id: Hash + Copy + Eq;
    fn get_id(&self) -> Self::Id;
    fn set_id(&mut self, id: Self::Id);
}