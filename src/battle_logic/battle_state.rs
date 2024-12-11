use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event::BattleEvent;
use crate::entities::battle_monster::BattleMonster;
use crate::enums::event_target::EventTarget;
use crate::enums::team_side::TeamSide;
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
        action_index: usize,
        source_team: &TeamSide,
        target_team: &TeamSide,
        source_monster_index: usize,
        target_monster_index: usize,
    ) -> Result<(), BattleError> {
        let event = Self::prepare_action(
            self,
            action_index,
            source_team,
            target_team,
            source_monster_index,
            target_monster_index,
        )?;
        event.process(self)
    }

    pub fn prepare_action(
        &self,
        action_index: usize,
        source_team: &TeamSide,
        target_team: &TeamSide,
        source_monster_index: usize,
        target_monster_index: usize,
    ) -> Result<BattleEvent, BattleError> {
        let source_monster = self.get_monster(source_team, source_monster_index).ok_or(BattleError::InvalidSourceMonsterIndex)?;
        self.get_monster(&source_team.opposite(), target_monster_index).ok_or(BattleError::InvalidTargetMonsterIndex)?;

        let action = source_monster.get_action(action_index).ok_or(BattleError::InvalidActionIndex)?;
        if !action.validate_target(source_team, target_team, source_monster_index, target_monster_index) {
            return Err(BattleError::InvalidActionTarget);
        }

        Ok(BattleEvent::from_action(
            action,
            action_index,
            source_team,
            target_team,
            source_monster_index,
            target_monster_index,
        ))
    }

    pub fn update_specific_monster<F>(&mut self, target_team: TeamSide, index: usize, update_fn: &F) -> Result<(), BattleError>
    where
        F: Fn(&mut BattleMonster) -> Result<(), BattleError>,
    {
        match target_team {
            TeamSide::TeamA => {
                if let Some(monster) = self.monsters_a.get_mut(index) {
                    update_fn(monster)
                } else {
                    Err(BattleError::InvalidMonsterIndex)
                }
            }
            TeamSide::TeamB => {
                if let Some(monster) = self.monsters_b.get_mut(index) {
                    update_fn(monster)
                } else {
                    Err(BattleError::InvalidMonsterIndex)
                }
            }
        }
    }

    pub fn update_monsters_of_a_team<F>(&mut self, target_team: TeamSide, update_fn: F) -> Result<(), BattleError>
    where
        F: Fn(&mut BattleMonster) -> Result<(), BattleError>,
    {
        match target_team {
            TeamSide::TeamA => {
                for i in 0..self.monsters_a.len() {
                    self.update_specific_monster(target_team, i, &update_fn)?;
                }
            }
            TeamSide::TeamB => {
                for i in 0..self.monsters_b.len() {
                    self.update_specific_monster(target_team, i, &update_fn)?;
                }
            }
        }
        Ok(())
    }

    pub fn update_monsters_by_event_target<F>(&mut self, source_team: TeamSide, target_team: TeamSide, source_monster_index: usize, target_monster_index: usize, event_target: EventTarget, update_fn: F) -> Result<(), BattleError>
    where
        F: Fn(&mut BattleMonster) -> Result<(), BattleError>,
    {
        match event_target {
            EventTarget::SourceMonster => {
                self.update_specific_monster(
                    source_team,
                    source_monster_index,
                    &update_fn,
                )
            }
            EventTarget::TargetMonster => {
                self.update_specific_monster(
                    target_team,
                    target_monster_index,
                    &update_fn,
                )
            }
            EventTarget::SourceTeam => {
                self.update_monsters_of_a_team(
                    source_team,
                    update_fn,
                )
            }
            EventTarget::TargetTeam => {
                self.update_monsters_of_a_team(
                    target_team,
                    update_fn,
                )
            }
        }
    }
}
