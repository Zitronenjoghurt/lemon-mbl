use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_cost::BattleEventCost;
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
    costs: Vec<BattleEventCost>,
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
            costs: Vec::from(action.get_costs()),
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

    pub fn check_costs(&self, state: &mut BattleState) -> Result<(), BattleError> {
        state
            .get_monster(&self.source_team, self.source_monster_index)
            .ok_or(BattleError::InvalidMonsterIndex)?
            .check_costs(&self.costs)
    }

    pub fn process_costs(&self, state: &mut BattleState) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        let mut cost_feedback_entries = Vec::new();
        for cost in &self.costs {
            let (_, feedback) = state.update_specific_monster(
                self.source_team,
                self.source_monster_index,
                &|m| {
                    m.process_cost(cost)?;
                    let feedback_entry = BattleEventFeedbackEntry {
                        target_team: self.source_team,
                        target_monster_index: self.source_monster_index,
                        feedback_type: cost.resource.get_used_feedback_type(),
                        value: Some(cost.amount as i64),
                        factor: None,
                    };
                    Ok(((), vec![feedback_entry]))
                },
            )?;
            cost_feedback_entries.extend(feedback);
        }
        Ok(cost_feedback_entries)
    }

    pub fn process_event_types(&mut self, state: &mut BattleState) -> Result<Vec<Vec<BattleEventFeedbackEntry>>, BattleError> {
        let mut feedback_entries = Vec::new();

        while let Some(mut event_type) = self.types.pop() {
            let mut event_type_feedback_entries = Vec::new();

            if !event_type.start_triggered() {
                let on_start_feedback_entries = event_type.on_start()?;
                event_type_feedback_entries.extend(on_start_feedback_entries);
            }

            let process_feedback_entries = event_type.process(
                state,
                self.source_team,
                self.target_team,
                self.source_monster_index,
                self.target_monster_index,
            )?;
            event_type_feedback_entries.extend(process_feedback_entries);

            if !event_type.should_remove() {
                self.types.push(event_type);
            } else {
                let on_end_feedback_entries = event_type.on_end()?;
                event_type_feedback_entries.extend(on_end_feedback_entries);
            }

            feedback_entries.push(event_type_feedback_entries);
        }

        feedback_entries.reverse();
        Ok(feedback_entries)
    }

    pub fn process(&mut self, state: &mut BattleState) -> Result<BattleEventFeedback, BattleError> {
        self.check_costs(state)?;

        let cost_feedback_entries = self.process_costs(state)?;
        let type_feedback_entries = self.process_event_types(state)?;

        let mut feedback_entries: Vec<Vec<BattleEventFeedbackEntry>> = Vec::new();
        feedback_entries.push(cost_feedback_entries);
        feedback_entries.extend(type_feedback_entries);

        if let Some(action_index) = self.action_index {
            state.update_specific_monster_without_feedback(
                self.source_team,
                self.source_monster_index,
                &|m| m.on_action_used(action_index),
            )?;
        }

        Ok(BattleEventFeedback {
            source: self.feedback_source.clone(),
            entries: feedback_entries,
        })
    }

    pub fn should_remove(&self) -> bool {
        self.types.is_empty()
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