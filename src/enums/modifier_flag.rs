use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum ModifierFlag {
    Poisoned = 0,
    Paralyzed = 1,
}