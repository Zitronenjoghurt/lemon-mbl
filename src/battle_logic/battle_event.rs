use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::{BattleEventFeedback, BattleEventFeedbackEntry, BattleEventFeedbackSource};
use crate::battle_logic::battle_event_type::BattleEventType;
use crate::battle_logic::battle_log::BattleLogActionEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::entities::stored_action::StoredAction;
use crate::enums::team_side::TeamSide;
use crate::traits::action_data_access::ActionDataAccess;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BattleEvent {
    types: Vec<BattleEventType>,
    feedback_source: BattleEventFeedbackSource,
    action_index: Option<usize>,
    source_team: TeamSide,
    target_team: TeamSide,
    source_monster_index: usize,
    target_monster_index: usize,
    priority: u8,
    secondary_priority: u16,
}

impl BattleEvent {
    pub fn from_action(
        action: &StoredAction,
        action_index: usize,
        source_team: &TeamSide,
        target_team: &TeamSide,
        source_monster_index: usize,
        target_monster_index: usize,
    ) -> Self {
        let feedback_source = BattleEventFeedbackSource {
            source_team: Some(*source_team),
            source_monster_index: Some(source_monster_index),
            action_id: Some(action.get_id()),
            action_index: Some(action_index),
        };

        Self {
            types: Vec::from(action.get_event_types()),
            feedback_source,
            action_index: Some(action_index),
            source_team: *source_team,
            target_team: *target_team,
            source_monster_index,
            target_monster_index,
            priority: action.get_priority(),
            secondary_priority: action.get_id(),
        }
    }

    pub fn get_log_action_entry(&self) -> Option<BattleLogActionEntry> {
        self.action_index.map(|action_index| BattleLogActionEntry {
            action_index,
            source_team: self.source_team,
            target_team: self.target_team,
            source_monster_index: self.source_monster_index,
            target_monster_index: self.target_monster_index,
        })
    }

    pub fn process(&self, state: &mut BattleState) -> Result<BattleEventFeedback, BattleError> {
        if let Some(action_index) = self.action_index {
            state.update_specific_monster_without_feedback(
                self.source_team,
                self.source_monster_index,
                &|m| m.on_action_used(action_index),
            )?;
        }

        let feedback_entries: Vec<Vec<BattleEventFeedbackEntry>> = self.types.iter()
            .map(|event_type| {
                event_type.process(
                    state,
                    self.source_team,
                    self.target_team,
                    self.source_monster_index,
                    self.target_monster_index,
                )
            })
            .collect::<Result<Vec<Vec<_>>, BattleError>>()?;

        Ok(BattleEventFeedback {
            source: self.feedback_source.clone(),
            entries: feedback_entries,
        })
    }
}

impl PartialOrd for BattleEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BattleEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.priority.cmp(&self.priority) {
            Ordering::Equal => {
                match other.secondary_priority.cmp(&self.secondary_priority) {
                    Ordering::Equal => {
                        (
                            &self.types,
                            &self.action_index,
                            &self.source_team,
                            &self.target_team,
                            &self.source_monster_index,
                            &self.target_monster_index,
                        ).cmp(&(
                            &other.types,
                            &other.action_index,
                            &other.source_team,
                            &other.target_team,
                            &other.source_monster_index,
                            &other.target_monster_index,
                        ))
                    }
                    other_ord => other_ord
                }
            }
            other_ord => other_ord
        }
    }
}