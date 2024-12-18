use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::battle_event_feedback_type::BattleEventFeedbackType;
use crate::enums::event_target::EventTarget;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HealEventType {
    pub amount: u32,
    pub target: EventTarget,
}

impl HealEventType {
    pub fn process(
        &self,
        state: &mut BattleState,
        source_team: TeamSide,
        target_team: TeamSide,
        source_index: usize,
        target_index: usize,
    ) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        let (hp_healed_cumulative, target_monsters_feedback) = state.update_monsters_by_event_target_with_accumulator(
            source_team,
            target_team,
            source_index,
            target_index,
            self.target,
            |m| {
                let hp_healed = m.process_heal(self.amount);
                let feedback_entry = BattleEventFeedbackEntry {
                    target_team,
                    target_monster_index: target_index,
                    feedback_type: BattleEventFeedbackType::HpHealReceived,
                    value: Some(hp_healed as i64),
                    factor: None,
                };
                Ok((hp_healed, vec![feedback_entry]))
            },
        )?;

        let (_, source_monster_feedback) = state.update_specific_monster(
            source_team,
            source_index,
            &|m| {
                m.on_hp_heal_given(hp_healed_cumulative);
                let feedback_entry = BattleEventFeedbackEntry {
                    target_team: source_team,
                    target_monster_index: source_index,
                    feedback_type: BattleEventFeedbackType::HpHealGiven,
                    value: Some(hp_healed_cumulative as i64),
                    factor: None,
                };
                Ok(((), vec![feedback_entry]))
            },
        )?;

        let mut feedback = Vec::new();
        feedback.extend(target_monsters_feedback);
        feedback.extend(source_monster_feedback);
        Ok(feedback)
    }
}