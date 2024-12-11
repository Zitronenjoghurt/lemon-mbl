use crate::calculations::stats::energy_from_potential_and_vigilance;
use crate::entities::monster_data::MonsterData;
use crate::entities::stored_action::StoredAction;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::monster_data_access::MonsterDataAccess;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleMonster {
    #[serde(with = "arc_ref")]
    data: Arc<MonsterData>,
    storage_data: StoredMonster,
    current_hp: u16,
    desperation: u16,
    energy: u16,
    momentum: u16,
}

impl BattleMonster {
    pub fn create(id: u16) -> Option<Self> {
        get_game_data().monsters.get(id)
            .map(|data| Self::from_data(Arc::clone(data)))
    }

    pub fn from_data(data: Arc<MonsterData>) -> Self {
        let energy = energy_from_potential_and_vigilance(data.get_potential(), data.get_vigilance());

        Self {
            current_hp: data.get_vitality(),
            storage_data: StoredMonster::from_data(data.clone()),
            desperation: 0,
            momentum: 0,
            energy,
            data,
        }
    }

    pub fn from_stored_monster(stored_monster: StoredMonster) -> Self {
        let data = stored_monster.get_data();
        let energy = energy_from_potential_and_vigilance(data.get_potential(), data.get_vigilance());

        Self {
            current_hp: stored_monster.get_current_hp(),
            storage_data: stored_monster,
            desperation: 0,
            momentum: 0,
            energy,
            data,
        }
    }

    pub fn get_action(&self, index: usize) -> Option<&StoredAction> {
        self.storage_data.get_action(index)
    }

    pub fn get_data(&self) -> Arc<MonsterData> {
        self.data.clone()
    }

    pub fn get_stored_data(&self) -> StoredMonster {
        self.storage_data.clone()
    }

    pub fn get_current_hp(&self) -> u16 {
        self.current_hp
    }

    pub fn set_current_hp(&mut self, hp: u16) {
        self.current_hp = hp;
    }

    pub fn get_max_hp(&self) -> u16 {
        self.data.get_vitality()
    }

    pub fn get_desperation(&self) -> u16 {
        self.desperation
    }

    pub fn set_desperation(&mut self, desperation: u16) {
        self.desperation = desperation;
    }

    pub fn get_momentum(&self) -> u16 {
        self.momentum
    }

    pub fn set_momentum(&mut self, momentum: u16) {
        self.momentum = momentum;
    }

    pub fn get_energy(&self) -> u16 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u16) {
        self.energy = energy;
    }
}

impl MonsterDataAccess for BattleMonster {
    fn get_id(&self) -> u16 {
        self.data.get_id()
    }

    fn get_internal_name(&self) -> &str {
        self.data.get_internal_name()
    }


    fn get_vitality(&self) -> u16 {
        self.data.get_vitality()
    }

    fn get_potential(&self) -> u16 {
        self.data.get_potential()
    }

    fn get_control(&self) -> u16 {
        self.data.get_control()
    }

    fn get_strength(&self) -> u16 {
        self.data.get_strength()
    }

    fn get_resilience(&self) -> u16 {
        self.data.get_resilience()
    }

    fn get_speed(&self) -> u16 {
        self.data.get_speed()
    }

    fn get_technique(&self) -> u16 {
        self.data.get_technique()
    }

    fn get_agility(&self) -> u16 {
        self.data.get_agility()
    }

    fn get_vigilance(&self) -> u16 {
        self.data.get_vigilance()
    }

    fn get_focus(&self) -> u16 {
        self.data.get_focus()
    }

    fn get_flags(&self) -> &[MonsterFlag] {
        self.data.get_flags()
    }

    fn has_flag(&self, flag: MonsterFlag) -> bool {
        self.data.has_flag(flag)
    }

    fn get_physical_types(&self) -> &[MonsterPhysicalType] {
        self.data.get_physical_types()
    }

    fn has_physical_type(&self, physical_type: MonsterPhysicalType) -> bool {
        self.data.has_physical_type(physical_type)
    }

    fn get_elemental_types(&self) -> &[MonsterElementalType] {
        self.data.get_elemental_types()
    }

    fn has_elemental_type(&self, elemental_type: MonsterElementalType) -> bool {
        self.data.has_elemental_type(elemental_type)
    }
}

impl From<StoredMonster> for BattleMonster {
    fn from(stored_monster: StoredMonster) -> Self {
        Self::from_stored_monster(stored_monster)
    }
}
