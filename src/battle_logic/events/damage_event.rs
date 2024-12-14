use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::battle_event_feedback_text::BattleEventFeedbackText;
use crate::enums::battle_event_feedback_type::BattleEventFeedbackType;
use crate::enums::damage_type::DamageType;
use crate::enums::event_target::EventTarget;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DamageEventType {
    pub amount: u32,
    pub damage_types: Vec<DamageType>,
    pub target: EventTarget,
}

impl DamageEventType {
    pub fn process(
        &self, state:
        &mut BattleState,
        source_team: TeamSide,
        target_team: TeamSide,
        source_index: usize,
        target_index: usize,
    ) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        let (damage_dealt_cumulative, target_monsters_feedback) = state.update_monsters_by_event_target_with_accumulator(
            source_team,
            target_team,
            source_index,
            target_index,
            self.target,
            |m| {
                let damage = m.process_damage(self.amount, &self.damage_types);
                let feedback_entry = BattleEventFeedbackEntry {
                    target_team,
                    target_monster_index: target_index,
                    feedback_type: BattleEventFeedbackType::RawDamageTaken,
                    feedback_text: BattleEventFeedbackText::DamageTaken,
                    value: damage as i64,
                };
                Ok((damage, vec![feedback_entry]))
            },
        )?;

        let (_, source_monster_feedback) = state.update_specific_monster(
            source_team,
            source_index,
            &|m| {
                m.on_damage_dealt(damage_dealt_cumulative);
                let feedback_entry = BattleEventFeedbackEntry {
                    target_team: source_team,
                    target_monster_index: source_index,
                    feedback_type: BattleEventFeedbackType::RawDamageDealt,
                    feedback_text: BattleEventFeedbackText::DamageDealt,
                    value: damage_dealt_cumulative as i64,
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