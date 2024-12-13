use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::event_target::EventTarget;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HealEventType {
    pub amount: u32,
    pub target: EventTarget,
}

impl HealEventType {
    pub fn process(&self, state: &mut BattleState, source_team: TeamSide, target_team: TeamSide, source_index: usize, target_index: usize) -> Result<(), BattleError> {
        let hp_healed_cumulative = state.update_monsters_by_event_target_with_accumulator(
            source_team,
            target_team,
            source_index,
            target_index,
            self.target,
            |m| {
                Ok(m.process_heal(self.amount))
            },
        )?;

        state.update_specific_monster(
            source_team,
            source_index,
            &|m| {
                m.on_hp_heal_given(hp_healed_cumulative);
                Ok(())
            },
        )
    }
}