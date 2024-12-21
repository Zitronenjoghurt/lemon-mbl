use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum AbilityTarget {
    None = 0,
    OneSelf = 1,
    RandomAlly = 2,
    RandomOpponent = 3,
    EveryAlly = 4,
    EveryAllyExceptSelf = 5,
    EveryOpponent = 6,
}