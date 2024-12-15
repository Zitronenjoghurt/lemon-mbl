use crate::enums::battle_event_feedback_text::BattleEventFeedbackText;
use crate::enums::battle_event_feedback_type::BattleEventFeedbackType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum ResourceType {
    Momentum = 0,
    Energy = 1,
    Hp = 2,
}

impl ResourceType {
    pub fn get_used_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Momentum => BattleEventFeedbackType::MomentumUsed,
            Self::Energy => BattleEventFeedbackType::EnergyUsed,
            Self::Hp => BattleEventFeedbackType::HpUsed,
        }
    }

    pub fn get_used_feedback_text(&self) -> BattleEventFeedbackText {
        match self {
            Self::Momentum => BattleEventFeedbackText::MomentumUsed,
            Self::Energy => BattleEventFeedbackText::EnergyUsed,
            Self::Hp => BattleEventFeedbackText::HpUsed,
        }
    }

    pub fn get_generation_received_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Momentum => BattleEventFeedbackType::MomentumReceived,
            Self::Energy => BattleEventFeedbackType::EnergyReceived,
            Self::Hp => BattleEventFeedbackType::HpHealReceived,
        }
    }

    pub fn get_generation_received_feedback_text(&self) -> BattleEventFeedbackText {
        match self {
            Self::Momentum => BattleEventFeedbackText::MomentumReceived,
            Self::Energy => BattleEventFeedbackText::EnergyReceived,
            Self::Hp => BattleEventFeedbackText::HpHealReceived,
        }
    }

    pub fn get_generation_given_feedback_type(&self) -> BattleEventFeedbackType {
        match self {
            Self::Momentum => BattleEventFeedbackType::MomentumGiven,
            Self::Energy => BattleEventFeedbackType::EnergyGiven,
            Self::Hp => BattleEventFeedbackType::HpHealGiven,
        }
    }

    pub fn get_generation_given_feedback_text(&self) -> BattleEventFeedbackText {
        match self {
            Self::Momentum => BattleEventFeedbackText::MomentumGiven,
            Self::Energy => BattleEventFeedbackText::EnergyGiven,
            Self::Hp => BattleEventFeedbackText::HpHealGiven
        }
    }
}