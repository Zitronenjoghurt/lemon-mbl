use crate::directories::monster_data_path;
use crate::enums::monster_flags::MonsterFlag;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterData {
    id: u16,
    #[serde(skip_deserializing)]
    internal_name: String,
    hp: u16,
    attack: u16,
    defense: u16,
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

    fn set_internal_name(&mut self, name: String) {
        self.internal_name = name;
    }
}

impl MonsterData {
    pub fn hp(&self) -> u16 {
        self.hp
    }

    pub fn attack(&self) -> u16 {
        self.attack
    }

    pub fn defense(&self) -> u16 {
        self.defense
    }

    pub fn flags(&self) -> &[MonsterFlag] {
        &self.flags
    }

    pub fn has_flag(&self, flag: MonsterFlag) -> bool {
        self.flags.contains(&flag)
    }
}