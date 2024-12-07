use crate::entities::monster_data::MonsterData;
use crate::entity_library::EntityLibrary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameData {
    pub monsters: EntityLibrary<MonsterData>,
}

impl GameData {
    pub fn load() -> Self {
        Self {
            monsters: EntityLibrary::from_yaml().unwrap(),
        }
    }
}