use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum ActionTarget {
    None = 0,
    SpecificAlly = 1,
    SpecificOpponent = 2,
    EveryAllyExceptSelf = 3,
    EveryAllyIncludingSelf = 4,
    EveryOpponent = 5,
}