use crate::data_library::DataLibrary;
use crate::data_objects::monster_data::MonsterData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameData {
    pub monsters: DataLibrary<MonsterData>,
}

impl GameData {
    pub fn load() -> Self {
        Self {
            monsters: DataLibrary::from_yaml().unwrap(),
        }
    }
}