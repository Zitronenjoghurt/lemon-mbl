use crate::data_structures::config_data::ConfigData;
use crate::data_structures::damage_type_library::DamageTypeLibrary;
use crate::data_structures::data_library::DataLibrary;
use crate::data_structures::monster_images::MonsterImages;
use crate::entities::action_data::ActionData;
use crate::entities::monster_data::MonsterData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameData {
    pub monsters: DataLibrary<MonsterData>,
    pub actions: DataLibrary<ActionData>,
    pub damage_types: DamageTypeLibrary,
    pub monster_images: MonsterImages,
    pub config: ConfigData,
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            monsters: DataLibrary::from_yaml()?,
            actions: DataLibrary::from_yaml()?,
            damage_types: DamageTypeLibrary::from_yaml()?,
            monster_images: MonsterImages::load()?,
            config: ConfigData::from_yaml()?,
        })
    }
}