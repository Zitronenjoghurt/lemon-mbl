use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::event_target::EventTarget;
use crate::enums::resource_type::ResourceType;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenerateResourceEventType {
    pub resource: ResourceType,
    pub amount: u32,
    pub target: EventTarget,
}

impl GenerateResourceEventType {
    pub fn process(
        &self,
        state: &mut BattleState,
        source_team: TeamSide,
        target_team: TeamSide,
        source_index: usize,
        target_index: usize,
    ) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        let (amount_generated_cumulative, target_monsters_feedback) = state.update_monsters_by_event_target_with_accumulator(
            source_team,
            target_team,
            source_index,
            target_index,
            self.target,
            |m| {
                let amount_generated = m.process_resource_generation(self.resource, self.amount);
                let feedback_entry = BattleEventFeedbackEntry {
                    target_team,
                    target_monster_index: target_index,
                    feedback_type: self.resource.get_generation_received_feedback_type(),
                    feedback_text: self.resource.get_generation_received_feedback_text(),
                    value: Some(amount_generated as i64),
                    factor: None,
                };
                Ok((amount_generated, vec![feedback_entry]))
            },
        )?;

        let (_, source_monster_feedback) = state.update_specific_monster(
            source_team,
            source_index,
            &|m| {
                match self.resource {
                    ResourceType::Momentum => m.on_momentum_generated_for_others(amount_generated_cumulative),
                    ResourceType::Energy => m.on_energy_generated_for_others(amount_generated_cumulative),
                    ResourceType::Hp => m.on_hp_heal_given(amount_generated_cumulative),
                }
                let feedback_entry = BattleEventFeedbackEntry {
                    target_team: source_team,
                    target_monster_index: source_index,
                    feedback_type: self.resource.get_generation_given_feedback_type(),
                    feedback_text: self.resource.get_generation_given_feedback_text(),
                    value: Some(amount_generated_cumulative as i64),
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