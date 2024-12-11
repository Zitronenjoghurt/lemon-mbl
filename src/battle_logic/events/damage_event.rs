use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_state::BattleState;
use crate::enums::damage_type::DamageType;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageEventType {
    pub amount: u16,
    pub damage_types: Vec<DamageType>,
}

impl DamageEventType {
    pub fn process(&self, state: &mut BattleState, source_team: TeamSide, source_index: usize, target_index: usize) -> Result<(), BattleError> {
        Ok(())
    }
}