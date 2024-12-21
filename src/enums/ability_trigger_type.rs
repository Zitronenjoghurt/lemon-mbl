use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum AbilityTriggerType {
    Never = 0,
    BattleStart = 1,
    EveryTurn = 2,
}