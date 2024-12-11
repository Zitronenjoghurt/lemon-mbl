use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event::BattleEvent;
use crate::entities::battle_monster::BattleMonster;
use crate::enums::action_target::ActionTarget;
use crate::enums::team_side::TeamSide;
use crate::traits::action_data_access::ActionDataAccess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleState {
    turn_counter: u16,
    is_team_a_turn: bool,
    monsters_a: Vec<BattleMonster>,
    monsters_b: Vec<BattleMonster>,
}

impl BattleState {
    pub fn new(team_a_monsters: Vec<BattleMonster>, team_b_monsters: Vec<BattleMonster>) -> Self {
        Self {
            turn_counter: 0,
            is_team_a_turn: true,
            monsters_a: team_a_monsters,
            monsters_b: team_b_monsters,
        }
    }

    pub fn get_monster(&self, team_side: &TeamSide, index: usize) -> Option<&BattleMonster> {
        match team_side {
            TeamSide::TeamA => self.monsters_a.get(index),
            TeamSide::TeamB => self.monsters_b.get(index),
        }
    }

    pub fn take_action(
        &mut self,
        acting_team: &TeamSide,
        source_monster_index: usize,
        action_index: usize,
        action_target: &ActionTarget,
        target_monster_index: usize,
    ) -> Result<(), BattleError> {
        let event = Self::process_action(
            self,
            acting_team,
            source_monster_index,
            action_index,
            action_target,
            target_monster_index,
        )?;
        event.process(self)
    }

    pub fn process_action(
        &self,
        acting_team: &TeamSide,
        source_monster_index: usize,
        action_index: usize,
        action_target: &ActionTarget,
        target_monster_index: usize,
    ) -> Result<BattleEvent, BattleError> {
        let source_monster = self.get_monster(acting_team, source_monster_index).ok_or(BattleError::InvalidSourceMonsterIndex)?;
        self.get_monster(&acting_team.opposite(), target_monster_index).ok_or(BattleError::InvalidTargetMonsterIndex)?;

        let action = source_monster.get_action(action_index).ok_or(BattleError::InvalidActionIndex)?;
        if !action.has_potential_target(action_target) {
            return Err(BattleError::InvalidActionTarget);
        }

        Ok(BattleEvent::from_action(
            action,
            acting_team,
            action_target,
            action_index,
            source_monster_index,
            target_monster_index,
        ))
    }
}