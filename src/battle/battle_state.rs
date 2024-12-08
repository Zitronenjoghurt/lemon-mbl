use crate::entities::battle_monster::BattleMonster;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    turn_counter: u16,
    is_team_a_turn: bool,
    monsters_a: Vec<BattleMonster>,
    monsters_b: Vec<BattleMonster>,
}