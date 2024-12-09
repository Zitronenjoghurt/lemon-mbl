use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum DamageType {
    Piercing = 0,
    Bludgeoning = 1,
    Slashing = 2,
    Fire = 3,
    Frost = 4,
    Thunder = 5,
    Light = 6,
    Dark = 7,
}