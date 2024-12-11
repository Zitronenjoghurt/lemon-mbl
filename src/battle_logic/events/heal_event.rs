use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealEventType {
    pub amount: u16,
}

impl HealEventType {
    pub fn process(&self, state: &mut BattleState, source_team: TeamSide, source_index: usize, target_index: usize) -> Result<(), BattleError> {
        Ok(())
    }
}