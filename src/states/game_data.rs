use crate::data_structures::data_library::DataLibrary;
use crate::data_structures::monster_images::MonsterImages;
use crate::entities::action_data::ActionData;
use crate::entities::monster_data::MonsterData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameData {
    pub monsters: DataLibrary<MonsterData>,
    pub actions: DataLibrary<ActionData>,
    pub monster_images: MonsterImages,
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            monsters: DataLibrary::from_yaml()?,
            actions: DataLibrary::from_yaml()?,
            monster_images: MonsterImages::load()?,
        })
    }
}