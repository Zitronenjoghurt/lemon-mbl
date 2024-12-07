use crate::directories::monster_data_path;
use crate::enums::monster_flags::MonsterFlag;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    id: u16,
    hp: u16,
    attack: u16,
    defense: u16,
    flags: Vec<MonsterFlag>,
}

impl HasDataFileYaml for Monster {
    fn data_file_path() -> PathBuf {
        monster_data_path()
    }
}

impl HasId for Monster {
    type Id = u16;

    fn id(&self) -> u16 {
        self.id
    }
}

impl Monster {
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