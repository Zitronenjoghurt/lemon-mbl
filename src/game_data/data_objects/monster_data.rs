use crate::enums::monster_flags::MonsterFlag;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::utils::directories::monster_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

impl MonsterData {
    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_internal_name(&self) -> &str {
        &self.internal_name
    }

    pub fn get_vitality(&self) -> u16 {
        self.vitality
    }

    pub fn get_potential(&self) -> u16 {
        self.potential
    }

    pub fn get_control(&self) -> u16 {
        self.control
    }

    pub fn get_strength(&self) -> u16 {
        self.strength
    }

    pub fn get_resilience(&self) -> u16 {
        self.resilience
    }

    pub fn get_speed(&self) -> u16 {
        self.speed
    }

    pub fn get_technique(&self) -> u16 {
        self.technique
    }

    pub fn get_agility(&self) -> u16 {
        self.agility
    }

    pub fn get_vigilance(&self) -> u16 {
        self.vigilance
    }

    pub fn get_focus(&self) -> u16 {
        self.focus
    }

    pub fn get_flags(&self) -> &[MonsterFlag] {
        &self.flags
    }

    pub fn get_has_flag(&self, flag: MonsterFlag) -> bool {
        self.flags.contains(&flag)
    }
}