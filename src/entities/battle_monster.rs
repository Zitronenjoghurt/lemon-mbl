use crate::calculations::stats::energy_from_potential_and_vigilance;
use crate::entities::monster_data::MonsterData;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flags::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleMonster {
    #[serde(with = "arc_ref")]
    data: Arc<MonsterData>,
    current_hp: u16,
    desperation: u16,
    energy: u16,
    momentum: u16,
}

impl BattleMonster {
    pub fn from_data(data: Arc<MonsterData>) -> Self {
        let energy = energy_from_potential_and_vigilance(data.get_potential(), data.get_vigilance());

        Self {
            current_hp: data.get_vitality(),
            desperation: 0,
            momentum: 0,
            energy,
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

    pub fn get_id(&self) -> u16 {
        self.data.id()
    }

    pub fn get_internal_name(&self) -> &str {
        self.data.internal_name()
    }

    pub fn get_max_hp(&self) -> u16 {
        self.data.get_vitality()
    }

    pub fn get_potential(&self) -> u16 {
        self.data.get_potential()
    }

    pub fn get_control(&self) -> u16 {
        self.data.get_control()
    }

    pub fn get_strength(&self) -> u16 {
        self.data.get_strength()
    }

    pub fn get_resilience(&self) -> u16 {
        self.data.get_resilience()
    }

    pub fn get_speed(&self) -> u16 {
        self.data.get_speed()
    }

    pub fn get_technique(&self) -> u16 {
        self.data.get_technique()
    }

    pub fn get_agility(&self) -> u16 {
        self.data.get_agility()
    }

    pub fn get_vigilance(&self) -> u16 {
        self.data.get_vigilance()
    }

    pub fn get_focus(&self) -> u16 {
        self.data.get_focus()
    }

    pub fn get_flags(&self) -> &[MonsterFlag] {
        self.data.get_flags()
    }

    pub fn has_flag(&self, flag: MonsterFlag) -> bool {
        self.data.has_flag(flag)
    }

    pub fn get_physical_types(&self) -> &[MonsterPhysicalType] {
        self.data.get_physical_types()
    }

    pub fn has_physical_type(&self, physical_type: MonsterPhysicalType) -> bool {
        self.data.has_physical_type(physical_type)
    }

    pub fn get_elemental_types(&self) -> &[MonsterElementalType] {
        self.data.get_elemental_types()
    }

    pub fn has_elemental_type(&self, elemental_type: MonsterElementalType) -> bool {
        self.data.has_elemental_type(elemental_type)
    }
}