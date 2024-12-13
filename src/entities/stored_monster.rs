use crate::entities::battle_monster::BattleMonster;
use crate::entities::monster_data::MonsterData;
use crate::entities::stored_action::StoredAction;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::has_assignable_id::HasAssignableId;
use crate::traits::monster_data_access::MonsterDataAccess;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredMonster {
    #[serde(with = "arc_ref")]
    data: Arc<MonsterData>,
    actions: Vec<StoredAction>,
    storage_id: u64,
    current_hp: u32,
    total_damage_taken: u32,
    total_damage_dealt: u32,
    total_hp_heal_given: u32,
    total_hp_heal_received: u32,
}

impl HasAssignableId for StoredMonster {
    type Id = u64;

    fn get_id(&self) -> Self::Id {
        self.storage_id
    }

    fn set_id(&mut self, id: Self::Id) {
        self.storage_id = id;
    }
}

impl StoredMonster {
    pub fn create(id: u16) -> Option<Self> {
        get_game_data().monsters.get(id)
            .map(|data| Self::from_data(Arc::clone(data)))
    }

    pub fn from_data(data: Arc<MonsterData>) -> Self {
        Self {
            current_hp: data.get_vitality(),
            actions: Vec::new(),
            storage_id: 0,
            total_damage_dealt: 0,
            total_damage_taken: 0,
            total_hp_heal_given: 0,
            total_hp_heal_received: 0,
            data,
        }
    }

    pub fn from_battle_monster(battle_monster: BattleMonster) -> Self {
        let stored_monster = battle_monster.get_stored_data();

        Self {
            current_hp: battle_monster.get_current_hp(),
            actions: stored_monster.actions,
            storage_id: stored_monster.storage_id,
            total_damage_dealt: stored_monster.total_damage_dealt,
            total_damage_taken: stored_monster.total_damage_taken,
            total_hp_heal_given: stored_monster.total_hp_heal_given,
            total_hp_heal_received: stored_monster.total_hp_heal_received,
            data: stored_monster.data,
        }
    }

    pub fn get_storage_id(&self) -> u64 {
        self.storage_id
    }

    pub fn get_data(&self) -> Arc<MonsterData> {
        self.data.clone()
    }

    pub fn get_current_hp(&self) -> u32 {
        self.current_hp
    }

    pub fn get_action(&self, index: usize) -> Option<&StoredAction> {
        self.actions.get(index)
    }

    pub fn get_action_mut(&mut self, index: usize) -> Option<&mut StoredAction> {
        self.actions.get_mut(index)
    }

    pub fn get_actions(&self) -> &Vec<StoredAction> {
        &self.actions
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<StoredAction> {
        &mut self.actions
    }

    pub fn learn_action(&mut self, action: StoredAction) {
        self.actions.push(action);
    }

    pub fn get_total_damage_dealt(&self) -> u32 {
        self.total_damage_dealt
    }

    pub fn get_total_damage_taken(&self) -> u32 {
        self.total_damage_taken
    }

    pub fn get_total_hp_heal_given(&self) -> u32 {
        self.total_hp_heal_given
    }

    pub fn get_total_hp_heal_received(&self) -> u32 {
        self.total_hp_heal_received
    }

    pub fn on_damage_dealt(&mut self, damage: u32) {
        self.total_damage_dealt = self.total_damage_dealt.saturating_add(damage);
    }

    pub fn on_damage_taken(&mut self, damage: u32) {
        self.total_damage_taken = self.total_damage_taken.saturating_add(damage);
    }

    pub fn on_hp_heal_given(&mut self, hp_healed: u32) {
        self.total_hp_heal_given = self.total_hp_heal_given.saturating_add(hp_healed);
    }

    pub fn on_hp_heal_received(&mut self, hp_healed: u32) {
        self.total_hp_heal_received = self.total_hp_heal_received.saturating_add(hp_healed);
    }
}

impl MonsterDataAccess for StoredMonster {
    fn get_id(&self) -> u16 {
        self.data.get_id()
    }

    fn get_internal_name(&self) -> &str {
        self.data.get_internal_name()
    }


    fn get_vitality(&self) -> u32 {
        self.data.get_vitality()
    }

    fn get_potential(&self) -> u32 {
        self.data.get_potential()
    }

    fn get_control(&self) -> u32 {
        self.data.get_control()
    }

    fn get_strength(&self) -> u32 {
        self.data.get_strength()
    }

    fn get_resilience(&self) -> u32 {
        self.data.get_resilience()
    }

    fn get_speed(&self) -> u32 {
        self.data.get_speed()
    }

    fn get_technique(&self) -> u32 {
        self.data.get_technique()
    }

    fn get_agility(&self) -> u32 {
        self.data.get_agility()
    }

    fn get_vigilance(&self) -> u32 {
        self.data.get_vigilance()
    }

    fn get_focus(&self) -> u32 {
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

impl From<BattleMonster> for StoredMonster {
    fn from(battle_monster: BattleMonster) -> Self {
        Self::from_battle_monster(battle_monster)
    }
}