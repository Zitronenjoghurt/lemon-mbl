use crate::enums::modifier_flag::ModifierFlag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub enum StatusEffect {
    Poisoned = 0,
    Paralyzed = 1,
}

impl StatusEffect {
    pub fn get_modifier_flag(&self) -> ModifierFlag {
        match self {
            Self::Poisoned => ModifierFlag::Poisoned,
            Self::Paralyzed => ModifierFlag::Paralyzed
        }
    }
}