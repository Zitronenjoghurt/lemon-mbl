use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum DamageType {
    None = 0,
    Piercing = 1,
    Bludgeoning = 2,
    Slashing = 3,
    Fire = 4,
    Frost = 5,
    Thunder = 6,
    Light = 7,
    Dark = 8,
}

impl Default for DamageType {
    fn default() -> Self {
        DamageType::None
    }
}