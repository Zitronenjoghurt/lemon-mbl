use crate::data_structures::config_data::ConfigData;
use crate::data_structures::damage_type_library::DamageTypeLibrary;
use crate::data_structures::data_library::DataLibrary;
use crate::data_structures::i18n::I18n;
use crate::data_structures::monster_images::MonsterImages;
use crate::entities::ability_data::AbilityData;
use crate::entities::action_data::ActionData;
use crate::entities::monster_data::MonsterData;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "dev", derive(Clone))]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameData {
    pub monsters: DataLibrary<MonsterData>,
    pub actions: DataLibrary<ActionData>,
    pub abilities: DataLibrary<AbilityData>,
    pub damage_types: DamageTypeLibrary,
    pub monster_images: MonsterImages,
    pub config: ConfigData,
    pub i18n: I18n,
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            monsters: DataLibrary::from_json()?,
            actions: DataLibrary::from_json()?,
            abilities: DataLibrary::from_json()?,
            damage_types: DamageTypeLibrary::from_json()?,
            monster_images: MonsterImages::load()?,
            config: ConfigData::from_yaml()?,
            i18n: I18n::load()?,
        })
    }
}

#[cfg(feature = "dev")]
mod dev {
    use crate::states::game_data::GameData;
    use std::fs;
    use std::path::PathBuf;

    impl GameData {
        pub fn dump(&self, data_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
            let monsters_yaml = self.monsters.to_json()?;
            let actions_yaml = self.actions.to_json()?;
            let abilities_yaml = self.abilities.to_json()?;

            let stats_path = data_path.join("stats");
            fs::create_dir_all(&stats_path)?;
            fs::write(stats_path.join("monsters.json"), &monsters_yaml)?;
            fs::write(stats_path.join("actions.json"), &actions_yaml)?;
            fs::write(stats_path.join("abilities.json"), &abilities_yaml)?;
            Ok(())
        }
    }
}
