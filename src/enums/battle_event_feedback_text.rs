use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u16)]
pub enum BattleEventFeedbackText {
    DamageTaken = 0,
    DamageDealt = 1,
    HpHealReceived = 2,
    HpHealGiven = 3,
    MomentumUsed = 4,
    EnergyUsed = 5,
    HpUsed = 6,
}