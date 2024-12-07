use crate::get_game_data;
use lemon_mbl_game_data::data_objects::monster_data::MonsterData;
use lemon_mbl_game_data::enums::monster_flags::MonsterFlag;
use lemon_mbl_game_data::traits::has_id::HasId;
use lemon_mbl_game_data::traits::has_internal_name::HasInternalName;
use std::sync::Arc;

pub struct BattleMonster {
    data: Arc<MonsterData>,
    current_hp: u16,
}

impl BattleMonster {
    pub fn from_data(data: Arc<MonsterData>) -> Self {
        Self {
            current_hp: data.hp(),
            data,
        }
    }

    pub fn create(id: u16) -> Option<Self> {
        get_game_data().monsters.get(id)
            .map(|data| Self::from_data(Arc::clone(data)))
    }

    pub fn get_current_hp(&self) -> u16 {
        self.current_hp
    }

    pub fn set_current_hp(&mut self, hp: u16) {
        self.current_hp = hp;
    }

    pub fn get_id(&self) -> u16 {
        self.data.id()
    }

    pub fn get_internal_name(&self) -> &str {
        self.data.internal_name()
    }

    pub fn get_max_hp(&self) -> u16 {
        self.data.hp()
    }

    pub fn get_attack(&self) -> u16 {
        self.data.attack()
    }

    pub fn get_defense(&self) -> u16 {
        self.data.defense()
    }

    pub fn get_flags(&self) -> &[MonsterFlag] {
        self.data.flags()
    }

    pub fn has_flag(&self, flag: MonsterFlag) -> bool {
        self.data.has_flag(flag)
    }
}