use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u16)]
pub enum BattleEventFeedbackType {
    RawDamageTaken = 0,
    RawDamageDealt = 1,
    HpHealReceived = 2,
    HpHealGiven = 3,
    MomentumUsed = 4,
    EnergyUsed = 5,
    HpUsed = 6,
    MomentumReceived = 7,
    EnergyReceived = 8,
    MomentumGiven = 9,
    EnergyGiven = 10,
    MomentumGeneratedEnergy = 11,
    PoisonApplied = 12,
    ParalysisApplied = 13,
    PoisonMissed = 14,
    ParalysisMissed = 15,
    PoisonAlreadyApplied = 16,
    ParalysisAlreadyApplied = 17,
    PoisonDamageTaken = 18,
    ParalyzedCantAct = 19,
    PoisonExpired = 20,
    ParalysisExpired = 21,
    PoisonExtended = 22,
    ParalysisExtended = 23,
}