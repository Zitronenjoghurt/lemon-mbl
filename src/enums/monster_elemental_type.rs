use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum MonsterElementalType {
    Force = 0,
    Fire = 1,
    Frost = 2,
    Thunder = 3,
    Light = 4,
    Dark = 5,
}