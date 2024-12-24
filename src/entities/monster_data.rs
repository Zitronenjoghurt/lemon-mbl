use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use crate::serialization::arc_ref::ArcRefFromKey;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::traits::monster_data_access::MonsterDataAccess;
use crate::utils::directories::monster_data_path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonsterData {
    id: u16,
    #[serde(default)]
    internal_name: String,
    vitality: u32,
    potential: u32,
    control: u32,
    strength: u32,
    resilience: u32,
    speed: u32,
    technique: u32,
    agility: u32,
    vigilance: u32,
    focus: u32,
    flags: Vec<MonsterFlag>,
    physical_types: Vec<MonsterPhysicalType>,
    elemental_types: Vec<MonsterElementalType>,
}

impl ArcRefFromKey for MonsterData {
    type Key = u16;

    fn to_key(&self) -> Self::Key {
        self.get_id()
    }

    fn from_key(key: &u16) -> Option<Arc<Self>>
    where
        Self: Sized,
    {
        get_game_data().monsters.get(*key).cloned()
    }
}

impl HasDataFileYaml for MonsterData {
    fn data_file_path() -> PathBuf {
        monster_data_path()
    }

    #[cfg(feature = "dev")]
    fn postprocess(contents: String) -> String {
        let internal_name_pattern = r"(?m)^ {2}internal_name:.*\n";
        let regex = Regex::new(internal_name_pattern).unwrap();
        regex.replace_all(&contents, "").to_string()
    }
}

impl HasId for MonsterData {
    type Id = u16;

    fn id(&self) -> u16 {
        self.get_id()
    }
}

impl HasInternalName for MonsterData {
    fn internal_name(&self) -> &str {
        self.get_internal_name()
    }

    fn with_internal_name(self, name: String) -> Self {
        Self {
            internal_name: name,
            ..self
        }
    }
}

impl MonsterDataAccess for MonsterData {
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_internal_name(&self) -> &str {
        &self.internal_name
    }

    fn get_vitality(&self) -> u32 {
        self.vitality
    }

    fn get_potential(&self) -> u32 {
        self.potential
    }

    fn get_control(&self) -> u32 {
        self.control
    }

    fn get_strength(&self) -> u32 {
        self.strength
    }

    fn get_resilience(&self) -> u32 {
        self.resilience
    }

    fn get_speed(&self) -> u32 {
        self.speed
    }

    fn get_technique(&self) -> u32 {
        self.technique
    }

    fn get_agility(&self) -> u32 {
        self.agility
    }

    fn get_vigilance(&self) -> u32 {
        self.vigilance
    }

    fn get_focus(&self) -> u32 {
        self.focus
    }

    fn get_flags(&self) -> &[MonsterFlag] {
        &self.flags
    }

    fn has_flag(&self, flag: MonsterFlag) -> bool {
        self.flags.contains(&flag)
    }

    fn get_physical_types(&self) -> &[MonsterPhysicalType] {
        &self.physical_types
    }

    fn has_physical_type(&self, physical_type: MonsterPhysicalType) -> bool {
        self.physical_types.contains(&physical_type)
    }

    fn get_elemental_types(&self) -> &[MonsterElementalType] {
        &self.elemental_types
    }

    fn has_elemental_type(&self, elemental_type: MonsterElementalType) -> bool {
        self.elemental_types.contains(&elemental_type)
    }
}