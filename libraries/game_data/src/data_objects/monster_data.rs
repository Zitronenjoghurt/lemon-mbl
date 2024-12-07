use crate::enums::monster_flags::MonsterFlag;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use lemon_mbl_utils::directories::monster_data_path;
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
        self.id
    }
}

impl HasInternalName for MonsterData {
    fn internal_name(&self) -> &str {
        &self.internal_name
    }

    fn with_internal_name(self, name: String) -> Self {
        Self {
            internal_name: name,
            ..self
        }
    }
}

impl MonsterData {
    pub fn vitality(&self) -> u16 {
        self.vitality
    }

    pub fn potential(&self) -> u16 {
        self.potential
    }

    pub fn control(&self) -> u16 {
        self.control
    }

    pub fn strength(&self) -> u16 {
        self.strength
    }

    pub fn resilience(&self) -> u16 {
        self.resilience
    }

    pub fn speed(&self) -> u16 {
        self.speed
    }

    pub fn technique(&self) -> u16 {
        self.technique
    }

    pub fn agility(&self) -> u16 {
        self.agility
    }

    pub fn vigilance(&self) -> u16 {
        self.vigilance
    }

    pub fn focus(&self) -> u16 {
        self.focus
    }

    pub fn flags(&self) -> &[MonsterFlag] {
        &self.flags
    }

    pub fn has_flag(&self, flag: MonsterFlag) -> bool {
        self.flags.contains(&flag)
    }
}