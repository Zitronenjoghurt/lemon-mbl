use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flags::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use crate::serialization::arc_ref::ArcRefFromKey;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::traits::monster_data_access::MonsterDataAccess;
use crate::utils::directories::monster_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonsterData {
    id: u16,
    #[serde(default)]
    internal_name: String,
    vitality: u16,
    potential: u16,
    control: u16,
    strength: u16,
    resilience: u16,
    speed: u16,
    technique: u16,
    agility: u16,
    vigilance: u16,
    focus: u16,
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

    fn get_vitality(&self) -> u16 {
        self.vitality
    }

    fn get_potential(&self) -> u16 {
        self.potential
    }

    fn get_control(&self) -> u16 {
        self.control
    }

    fn get_strength(&self) -> u16 {
        self.strength
    }

    fn get_resilience(&self) -> u16 {
        self.resilience
    }

    fn get_speed(&self) -> u16 {
        self.speed
    }

    fn get_technique(&self) -> u16 {
        self.technique
    }

    fn get_agility(&self) -> u16 {
        self.agility
    }

    fn get_vigilance(&self) -> u16 {
        self.vigilance
    }

    fn get_focus(&self) -> u16 {
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