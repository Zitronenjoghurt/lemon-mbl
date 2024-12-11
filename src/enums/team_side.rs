use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum TeamSide {
    TeamA = 0,
    TeamB = 1,
}

impl TeamSide {
    pub fn opposite(&self) -> Self {
        match self {
            TeamSide::TeamA => TeamSide::TeamB,
            TeamSide::TeamB => TeamSide::TeamA,
        }
    }
}