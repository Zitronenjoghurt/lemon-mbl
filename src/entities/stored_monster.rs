use crate::entities::monster_data::MonsterData;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::has_assignable_id::HasAssignableId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredMonster {
    #[serde(with = "arc_ref")]
    data: Arc<MonsterData>,
    storage_id: u64,
}

impl HasAssignableId for StoredMonster {
    type Id = u64;

    fn get_id(&self) -> Self::Id {
        self.storage_id
    }

    fn set_id(&mut self, id: Self::Id) {
        self.storage_id = id;
    }
}

impl StoredMonster {
    pub fn create(id: u16) -> Option<Self> {
        get_game_data().monsters.get(id)
            .map(|data| Self::from_data(Arc::clone(data)))
    }

    fn from_data(data: Arc<MonsterData>) -> Self {
        Self {
            data,
            storage_id: 0,
        }
    }

    pub fn get_storage_id(&self) -> u64 {
        self.storage_id
    }
}