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
    T: Clone + HasAssignableId + Serialize + DeserializeOwned,
    T::Id: Hash + Eq + Serialize + DeserializeOwned + From<u64> + TryInto<u64> + Ord,
{
    #[serde(with = "rw_lock")]
    items: RwLock<HashMap<T::Id, T>>,
    #[serde(with = "rw_lock")]
    next_id: RwLock<u64>,
}

impl<T> EntityCollection<T>
where
    T: Clone + HasAssignableId + Serialize + DeserializeOwned,
    T::Id: Eq + Hash + Serialize + DeserializeOwned + From<u64> + TryInto<u64> + Ord,
{
    pub fn new() -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
            next_id: RwLock::new(0),
        }
    }

    pub fn insert(&self, mut entity: T) {
        let mut write_guard = self.items.write().expect("Lock poisoned");
        let mut next_id_guard = self.next_id.write().expect("Lock poisoned");

        let id = T::Id::from(*next_id_guard);
        *next_id_guard += 1;

        entity.set_id(id);
        write_guard.insert(id, entity);
    }

    pub fn get(&self, id: &T::Id) -> Option<T> {
        let read_guard = self.items.read().expect("Lock poisoned");
        read_guard.get(id).cloned()
    }

    pub fn update<F>(&self, id: &T::Id, update_fn: F) -> bool
    where
        F: FnOnce(&mut T),
    {
        let mut write_guard = self.items.write().expect("Lock poisoned");

        if let Some(entity) = write_guard.get_mut(id) {
            update_fn(entity);
            true
        } else {
            false
        }
    }

    pub fn remove(&self, id: &T::Id) -> Option<T> {
        let mut write_guard = self.items.write().expect("Lock poisoned");
        write_guard.remove(id)
    }
}