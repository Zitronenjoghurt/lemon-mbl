use crate::enums::battle_event_feedback_source_type::BattleEventFeedbackSourceType;
use crate::enums::battle_event_feedback_type::BattleEventFeedbackType;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BattleEventFeedbackSource {
    pub source_type: BattleEventFeedbackSourceType,
    pub source_team: Option<TeamSide>,
    pub source_monster_index: Option<usize>,
    pub action_id: Option<u16>,
    pub action_index: Option<usize>,
}

impl BattleEventFeedbackSource {
    pub fn default_for_turn_end() -> Self {
        BattleEventFeedbackSource {
            source_type: BattleEventFeedbackSourceType::TurnEnd,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleEventFeedbackEntry {
    pub target_team: TeamSide,
    pub target_monster_index: usize,
    pub feedback_type: BattleEventFeedbackType,
    pub value: Option<i64>,
    pub factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleEventFeedback {
    pub source: BattleEventFeedbackSource,
    pub entries: Vec<Vec<BattleEventFeedbackEntry>>,
}