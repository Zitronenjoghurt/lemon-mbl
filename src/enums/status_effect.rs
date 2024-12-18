use crate::enums::battle_event_feedback_type::BattleEventFeedbackType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub enum StatusEffect {
    Poisoned = 0,
    Paralyzed = 1,
}

impl StatusEffect {
    pub fn get_apply_successful_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Poisoned => BattleEventFeedbackType::PoisonApplied,
            Self::Paralyzed => BattleEventFeedbackType::ParalysisApplied
        }
    }

    pub fn get_already_applied_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Poisoned => BattleEventFeedbackType::PoisonAlreadyApplied,
            Self::Paralyzed => BattleEventFeedbackType::ParalysisAlreadyApplied
        }
    }

    pub fn get_apply_missed_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Poisoned => BattleEventFeedbackType::PoisonMissed,
            Self::Paralyzed => BattleEventFeedbackType::ParalysisMissed
        }
    }

    pub fn get_expired_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Poisoned => BattleEventFeedbackType::PoisonExpired,
            Self::Paralyzed => BattleEventFeedbackType::ParalysisExpired
        }
    }

    pub fn get_extended_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Poisoned => BattleEventFeedbackType::PoisonExtended,
            Self::Paralyzed => BattleEventFeedbackType::ParalysisExtended
        }
    }
}