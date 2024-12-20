use crate::serialization::rw_lock;
use crate::traits::has_assignable_id::HasAssignableId;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityCollection<T>
where
    T: Clone + HasAssignableId + Serialize + DeserializeOwned + PartialEq,
    T::Id: Hash + Eq + Serialize + DeserializeOwned + From<u64> + TryInto<u64> + Ord + PartialEq,
{
    #[serde(with = "rw_lock")]
    items: RwLock<HashMap<T::Id, T>>,
    #[serde(with = "rw_lock")]
    next_id: RwLock<u64>,
}

impl<T> PartialEq for EntityCollection<T>
where
    T: Clone + HasAssignableId + Serialize + DeserializeOwned + PartialEq,
    T::Id: Hash + Eq + Serialize + DeserializeOwned + From<u64> + TryInto<u64> + Ord + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.next_id.read().unwrap() == *other.next_id.read().unwrap()
            && *self.items.read().unwrap() == *other.items.read().unwrap()
    }
}

impl<T> EntityCollection<T>
where
    T: Clone + HasAssignableId + Serialize + DeserializeOwned + PartialEq,
    T::Id: Eq + Hash + Serialize + DeserializeOwned + From<u64> + TryInto<u64> + Ord + PartialEq,
{
    pub fn new() -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        }
    }

    pub fn insert(&self, mut entity: T) -> T::Id {
        let mut write_guard = self.items.write().expect("Lock poisoned");
        let mut next_id_guard = self.next_id.write().expect("Lock poisoned");

        let id = T::Id::from(*next_id_guard);
        *next_id_guard += 1;

        entity.set_id(id);
        write_guard.insert(id, entity);
        id
    }

    pub fn get(&self, id: T::Id) -> Option<T> {
        let read_guard = self.items.read().expect("Lock poisoned");
        read_guard.get(&id).cloned()
    }

    pub fn update<F>(&self, id: T::Id, update_fn: F) -> bool
    where
        F: FnOnce(&mut T),
    {
        let mut write_guard = self.items.write().expect("Lock poisoned");

        if let Some(entity) = write_guard.get_mut(&id) {
            update_fn(entity);
            true
        } else {
            false
        }
    }

    pub fn remove(&self, id: T::Id) -> Option<T> {
        let mut write_guard = self.items.write().expect("Lock poisoned");
        write_guard.remove(&id)
    }
}