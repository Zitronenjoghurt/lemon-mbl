use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::calculations::int_bps::IntBps;
use crate::enums::event_target::EventTarget;
use crate::enums::status_effect::StatusEffect;
use crate::enums::team_side::TeamSide;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApplyStatusEffectEventType {
    pub effect: StatusEffect,
    pub chance: IntBps,
    pub target: EventTarget,
    pub min_turns: u8,
    pub max_turns: u8,
}

impl ApplyStatusEffectEventType {
    pub fn process(
        &mut self,
        state: &mut BattleState,
        source_team: TeamSide,
        target_team: TeamSide,
        source_index: usize,
        target_index: usize,
    ) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        if self.min_turns > self.max_turns {
            self.min_turns = self.max_turns;
        }

        let (times_applied, feedback) = state.update_monsters_by_event_target_with_accumulator(
            source_team,
            target_team,
            source_index,
            target_index,
            self.target,
            |monster| {
                let should_apply = self.chance.roll();

                let feedback = if should_apply {
                    let turns = thread_rng().gen_range(self.min_turns..=self.max_turns);
                    let already_had_effect = monster.has_status_effect(self.effect);

                    monster.add_status_effect(self.effect, turns);

                    let feedback_type = if already_had_effect {
                        self.effect.get_extended_feedback_type()
                    } else {
                        self.effect.get_apply_successful_feedback_type()
                    };

                    BattleEventFeedbackEntry {
                        target_team,
                        target_monster_index: target_index,
                        feedback_type,
                        value: Some(turns as i64),
                        factor: None,
                    }
                } else {
                    BattleEventFeedbackEntry {
                        target_team,
                        target_monster_index: 0,
                        feedback_type: self.effect.get_apply_missed_feedback_type(),
                        value: None,
                        factor: None,
                    }
                };

                let counter: u32 = if should_apply { 1 } else { 0 };
                Ok((counter, vec![feedback]))
            },
        )?;

        if times_applied > 0 {
            state.update_specific_monster_without_feedback(
                source_team,
                source_index,
                &|monster| {
                    match self.effect {
                        StatusEffect::Poisoned => monster.on_poison_applied(times_applied),
                        StatusEffect::Paralyzed => monster.on_paralysis_applied(times_applied),
                    };
                    Ok(())
                },
            )?;
        }

        Ok(feedback)
    }
}