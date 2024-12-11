use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::event_target::EventTarget;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HealEventType {
    pub amount: u16,
    pub target: EventTarget,
}

impl HealEventType {
    pub fn process(&self, state: &mut BattleState, source_team: TeamSide, target_team: TeamSide, source_index: usize, target_index: usize) -> Result<(), BattleError> {
        state.update_monsters_by_event_target(
            source_team,
            target_team,
            source_index,
            target_index,
            self.target,
            |m| {
                m.process_heal(self.amount)
            },
        )
    }
}