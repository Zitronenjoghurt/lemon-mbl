use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event::BattleEvent;
use crate::battle_logic::battle_log::{BattleLog, BattleLogActionEntry};
use crate::entities::battle_monster::BattleMonster;
use crate::enums::event_target::EventTarget;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleState {
    turn_counter: u16,
    monsters_a: Vec<BattleMonster>,
    monsters_b: Vec<BattleMonster>,
    team_a_moved: HashSet<usize>,
    team_b_moved: HashSet<usize>,
    current_turn_action_log: Vec<BattleLogActionEntry>,
    event_queue: BinaryHeap<BattleEvent>,
    battle_log: BattleLog,
}

// PartialEq implementation which ignores the BinaryHeap event queue
impl PartialEq for BattleState {
    fn eq(&self, other: &Self) -> bool {
        self.turn_counter == other.turn_counter &&
            self.monsters_a == other.monsters_a &&
            self.monsters_b == other.monsters_b &&
            self.team_a_moved == other.team_a_moved &&
            self.team_b_moved == other.team_b_moved &&
            self.current_turn_action_log == other.current_turn_action_log &&
            self.battle_log == other.battle_log
    }
}

impl BattleState {
    pub fn new(team_a_monsters: Vec<BattleMonster>, team_b_monsters: Vec<BattleMonster>) -> Self {
        Self {
            turn_counter: 1,
            monsters_a: team_a_monsters.clone(),
            monsters_b: team_b_monsters.clone(),
            team_a_moved: HashSet::new(),
            team_b_moved: HashSet::new(),
            current_turn_action_log: Vec::new(),
            event_queue: BinaryHeap::new(),
            battle_log: BattleLog::from_initial_data(
                team_a_monsters,
                team_b_monsters,
            ),
        }
    }

    pub fn get_monster(&self, team_side: &TeamSide, index: usize) -> Option<&BattleMonster> {
        match team_side {
            TeamSide::TeamA => self.monsters_a.get(index),
            TeamSide::TeamB => self.monsters_b.get(index),
        }
    }

    pub fn get_monsters(&self, team_side: &TeamSide) -> &Vec<BattleMonster> {
        match team_side {
            TeamSide::TeamA => &self.monsters_a,
            TeamSide::TeamB => &self.monsters_b
        }
    }

    pub fn get_current_turn(&self) -> u16 {
        self.turn_counter
    }

    pub fn queue_event(&mut self, event: BattleEvent) {
        self.event_queue.push(event);
    }

    pub fn process_event_queue(&mut self) -> Result<(), BattleError> {
        while let Some(event) = self.event_queue.pop() {
            event.process(self)?
        };

        self.battle_log.add_entry(
            self.get_current_turn(),
            self.current_turn_action_log.clone(),
            self.monsters_b.clone(),
            self.monsters_a.clone(),
        );

        self.team_a_moved.clear();
        self.team_b_moved.clear();
        self.current_turn_action_log.clear();
        self.turn_counter += 1;
        Ok(())
    }

    fn check_if_already_moved(&self, team: &TeamSide, monster_index: usize) -> Result<(), BattleError> {
        match team {
            TeamSide::TeamA => {
                if self.team_a_moved.contains(&monster_index) {
                    return Err(BattleError::AlreadyMoved);
                }
            }
            TeamSide::TeamB => {
                if self.team_b_moved.contains(&monster_index) {
                    return Err(BattleError::AlreadyMoved);
                }
            }
        }
        Ok(())
    }

    fn add_monster_to_moved(&mut self, team: &TeamSide, monster_index: usize) {
        match team {
            TeamSide::TeamA => self.team_a_moved.insert(monster_index),
            TeamSide::TeamB => self.team_b_moved.insert(monster_index)
        };
    }

    pub fn take_action(
        &mut self,
        action_index: usize,
        source_team: &TeamSide,
        target_team: &TeamSide,
        source_monster_index: usize,
        target_monster_index: usize,
    ) -> Result<(), BattleError> {
        self.check_if_already_moved(source_team, source_monster_index)?;

        let event = Self::prepare_action(
            self,
            action_index,
            source_team,
            target_team,
            source_monster_index,
            target_monster_index,
        )?;

        if let Some(entry) = event.get_log_action_entry() { self.current_turn_action_log.push(entry); }

        self.queue_event(event);
        self.add_monster_to_moved(source_team, source_monster_index);
        Ok(())
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
