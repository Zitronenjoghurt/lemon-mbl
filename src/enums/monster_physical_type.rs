use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum MonsterPhysicalType {
    Organic = 0,
    Slime = 1,
    Ethereal = 2,
    Construct = 3,
}