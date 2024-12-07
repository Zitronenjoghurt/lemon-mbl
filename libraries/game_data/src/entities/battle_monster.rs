use crate::entities::monster_data::MonsterData;
use std::sync::Arc;

pub struct BattleMonster {
    data: Arc<MonsterData>,
    current_hp: u16,
}

impl BattleMonster {
    pub fn new(data: Arc<MonsterData>) -> Self {
        Self {
            current_hp: data.hp(),
            data,
        }
    }
}