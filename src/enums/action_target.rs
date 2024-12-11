use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum ActionTarget {
    None = 0,
    OneSelf = 1,
    SpecificAlly = 2,
    SpecificOpponent = 3,
    EveryAllyExceptSelf = 4,
    EveryAllyIncludingSelf = 5,
    EveryOpponent = 6,
}