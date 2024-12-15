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
}