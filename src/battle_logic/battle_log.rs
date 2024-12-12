use crate::entities::battle_monster::BattleMonster;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleLogActionEntry {
    pub action_index: usize,
    pub source_team: TeamSide,
    pub target_team: TeamSide,
    pub source_monster_index: usize,
    pub target_monster_index: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BattleLogEntry {
    monsters_a: Vec<BattleMonster>,
    monsters_b: Vec<BattleMonster>,
    action_entries: Vec<BattleLogActionEntry>,
}

impl BattleLogEntry {
    pub fn create(monsters_a: Vec<BattleMonster>, monsters_b: Vec<BattleMonster>, action_entries: Vec<BattleLogActionEntry>) -> Self {
        Self {
            monsters_a,
            monsters_b,
            action_entries,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BattleLog {
    entries: HashMap<u16, BattleLogEntry>,
}

impl BattleLog {
    pub fn from_initial_data(monsters_a: Vec<BattleMonster>, monsters_b: Vec<BattleMonster>) -> Self {
        let entry = BattleLogEntry::create(monsters_a, monsters_b, vec![]);
        let mut log = BattleLog { entries: HashMap::new() };
        log.entries.insert(0, entry);
        log
    }

    pub fn add_entry(
        &mut self,
        current_turn: u16,
        action_entries: Vec<BattleLogActionEntry>,
        monsters_a: Vec<BattleMonster>,
        monsters_b: Vec<BattleMonster>,
    ) {
        let entry = BattleLogEntry::create(
            monsters_a,
            monsters_b,
            action_entries,
        );
        self.entries.entry(current_turn).or_insert(entry);
    }
}