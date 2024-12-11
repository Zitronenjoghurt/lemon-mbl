use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_type::BattleEventType;
use crate::battle_logic::battle_state::BattleState;
use crate::entities::stored_action::StoredAction;
use crate::enums::team_side::TeamSide;
use crate::traits::action_data_access::ActionDataAccess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleEvent {
    types: Vec<BattleEventType>,
    action_index: Option<usize>,
    source_team: TeamSide,
    target_team: TeamSide,
    source_monster_index: usize,
    target_monster_index: usize,
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
        Self {
            types: Vec::from(action.get_event_types()),
            action_index: Some(action_index),
            source_team: *source_team,
            target_team: *target_team,
            source_monster_index,
            target_monster_index,
        }
    }

    pub fn process(&self, state: &mut BattleState) -> Result<(), BattleError> {
        for event_type in self.types.iter() {
            event_type.process(
                state,
                self.source_team,
                self.target_team,
                self.source_monster_index,
                self.target_monster_index,
            )?;
        };
        Ok(())
    }
}