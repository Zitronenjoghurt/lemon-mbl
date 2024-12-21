use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum BattleEventFeedbackSourceType {
    None = 0,
    Action = 1,
    TurnEnd = 2,
    Ability = 3,
}

impl Default for BattleEventFeedbackSourceType {
    fn default() -> Self {
        Self::None
    }
}