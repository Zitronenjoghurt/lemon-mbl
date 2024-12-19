use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_cost::BattleEventCost;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::calculations::battle::calculate_poison_damage;
use crate::calculations::stats::energy_from_potential_and_vigilance;
use crate::entities::monster_data::MonsterData;
use crate::entities::monster_stats::MonsterStats;
use crate::entities::stored_action::StoredAction;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::battle_event_feedback_type::BattleEventFeedbackType;
use crate::enums::damage_type::DamageType;
use crate::enums::modifier_flag::ModifierFlag;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::enums::resource_type::ResourceType;
use crate::enums::status_effect::StatusEffect;
use crate::enums::team_side::TeamSide;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::monster_data_access::MonsterDataAccess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BattleMonster {
    #[serde(with = "arc_ref")]
    data: Arc<MonsterData>,
    storage_data: StoredMonster,
    modifier_flags: HashMap<ModifierFlag, u8>,
    status_effects: HashMap<StatusEffect, u8>,
    stats: MonsterStats,
    current_hp: u32,
    desperation: u32,
    energy: u32,
    momentum: u32,
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
            modifier_flags: HashMap::new(),
            status_effects: HashMap::new(),
            stats: MonsterStats::default(),
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
            modifier_flags: HashMap::new(),
            status_effects: HashMap::new(),
            stats: MonsterStats::default(),
            desperation: 0,
            momentum: 0,
            energy,
            data,
        }
    }

    pub fn get_action(&self, index: usize) -> Option<&StoredAction> {
        self.storage_data.get_action(index)
    }

    pub fn get_action_mut(&mut self, index: usize) -> Option<&mut StoredAction> {
        self.storage_data.get_action_mut(index)
    }

    pub fn get_data(&self) -> Arc<MonsterData> {
        self.data.clone()
    }

    pub fn get_stored_data(&self) -> StoredMonster {
        self.storage_data.clone()
    }

    pub fn get_modifier_flag(&self, flag: ModifierFlag) -> Option<u8> {
        self.modifier_flags.get(&flag).copied()
    }

    pub fn has_modifier_flag(&self, flag: ModifierFlag) -> bool {
        self.modifier_flags.contains_key(&flag)
    }

    pub fn get_modifier_flags(&self) -> &HashMap<ModifierFlag, u8> {
        &self.modifier_flags
    }

    pub fn add_modifier_flag(&mut self, flag: ModifierFlag) {
        self.modifier_flags
            .entry(flag)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn remove_modifier_flag(&mut self, flag: ModifierFlag) {
        if let Some(count) = self.modifier_flags.get_mut(&flag) {
            *count -= 1;
            if *count == 0 {
                self.modifier_flags.remove(&flag);
            }
        }
    }

    pub fn get_status_effects(&self) -> &HashMap<StatusEffect, u8> {
        &self.status_effects
    }

    pub fn has_status_effect(&self, effect: StatusEffect) -> bool {
        self.status_effects.contains_key(&effect)
    }

    pub fn add_status_effect(&mut self, effect: StatusEffect, turns: u8) {
        self.status_effects
            .entry(effect)
            .and_modify(|existing_turns| {
                if turns > *existing_turns {
                    *existing_turns = turns;
                }
            })
            .or_insert(turns);

        match effect {
            StatusEffect::Poisoned => self.on_poison_received(),
            StatusEffect::Paralyzed => self.on_paralysis_received()
        }
    }

    /// Returns true if the effect has been removed
    pub fn tick_status_effect(&mut self, effect: StatusEffect) -> bool {
        if let Some(turns_left) = self.status_effects.get_mut(&effect) {
            *turns_left = turns_left.saturating_sub(1);
            if *turns_left == 0 {
                self.remove_status_effect(effect);
                return true;
            }
        }
        false
    }

    pub fn remove_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects.remove(&effect);
    }

    pub fn get_stats(&self) -> &MonsterStats {
        &self.stats
    }

    pub fn get_current_hp(&self) -> u32 {
        self.current_hp
    }

    pub fn set_current_hp(&mut self, hp: u32) {
        self.current_hp = hp;
    }

    pub fn get_max_hp(&self) -> u32 {
        self.data.get_vitality()
    }

    pub fn get_desperation(&self) -> u32 {
        self.desperation
    }

    pub fn set_desperation(&mut self, desperation: u32) {
        self.desperation = desperation;
    }

    pub fn get_momentum(&self) -> u32 {
        self.momentum
    }

    pub fn set_momentum(&mut self, momentum: u32) {
        self.momentum = momentum;
    }

    pub fn get_energy(&self) -> u32 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u32) {
        self.energy = energy;
    }

    pub fn check_costs(&self, costs: &[BattleEventCost]) -> Result<(), BattleError> {
        for cost in costs {
            match cost.resource {
                ResourceType::Momentum => {
                    if self.get_momentum() < cost.amount {
                        return Err(BattleError::InsufficientMomentum);
                    }
                }
                ResourceType::Energy => {
                    if self.get_energy() < cost.amount {
                        return Err(BattleError::InsufficientEnergy);
                    }
                }
                ResourceType::Hp => {
                    if self.get_current_hp() < cost.amount {
                        return Err(BattleError::InsufficientHp);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn process_cost(&mut self, cost: &BattleEventCost) -> Result<(), BattleError> {
        match cost.resource {
            ResourceType::Momentum => {
                if self.get_momentum() < cost.amount {
                    return Err(BattleError::InsufficientMomentum);
                } else {
                    self.momentum = self.momentum.saturating_sub(cost.amount);
                    self.on_momentum_used(cost.amount);
                }
            }
            ResourceType::Energy => {
                if self.get_energy() < cost.amount {
                    return Err(BattleError::InsufficientEnergy);
                } else {
                    self.energy = self.energy.saturating_sub(cost.amount);
                    self.on_energy_used(cost.amount);
                }
            }
            ResourceType::Hp => {
                if self.get_current_hp() < cost.amount {
                    return Err(BattleError::InsufficientHp);
                } else {
                    self.current_hp = self.current_hp.saturating_sub(cost.amount);
                    self.on_hp_used(cost.amount);
                }
            }
        }
        Ok(())
    }

    pub fn process_costs(&mut self, costs: &[BattleEventCost]) -> Result<(), BattleError> {
        for cost in costs {
            self.process_cost(cost)?;
        }
        Ok(())
    }

    pub fn process_flat_damage(&mut self, amount: u32) -> u32 {
        let initial_hp = self.current_hp;
        self.current_hp = self.current_hp.saturating_sub(amount);

        let damage_taken = initial_hp - self.current_hp;
        self.on_damage_taken(damage_taken);
        damage_taken
    }

    pub fn process_damage(&mut self, amount: u32, damage_types: &[DamageType]) -> (u32, f64) {
        let damage_factor = get_game_data().damage_types.calculate_damage_factor(
            damage_types,
            self.get_physical_types(),
            self.get_elemental_types(),
        );

        let modified_amount = (amount as f64 * damage_factor)
            .round()
            .clamp(0.0, u32::MAX as f64) as u32;
        let damage_taken = self.process_flat_damage(modified_amount);

        (damage_taken, damage_factor)
    }

    pub fn process_heal(&mut self, amount: u32) -> u32 {
        let initial_hp = self.current_hp;
        self.current_hp = self.current_hp.saturating_add(amount).clamp(0, self.get_vitality());

        let hp_healed = self.current_hp - initial_hp;
        self.on_hp_heal_received(hp_healed);
        hp_healed
    }

    pub fn generate_momentum(&mut self, amount: u32) -> u32 {
        let initial_momentum = self.get_momentum();
        self.momentum = self.momentum.saturating_add(amount).clamp(0, self.get_control());

        let momentum_received = self.momentum - initial_momentum;
        self.on_momentum_generated(momentum_received);
        momentum_received
    }

    pub fn generate_energy(&mut self, amount: u32) -> u32 {
        let initial_energy = self.get_energy();
        self.energy = self.energy.saturating_add(amount).clamp(0, self.get_potential());

        let energy_received = self.energy - initial_energy;
        self.on_energy_generated(energy_received);
        energy_received
    }

    pub fn process_resource_generation(&mut self, resource: ResourceType, amount: u32) -> u32 {
        match resource {
            ResourceType::Momentum => {
                self.generate_momentum(amount)
            }
            ResourceType::Energy => {
                self.generate_energy(amount)
            }
            ResourceType::Hp => {
                self.process_heal(amount)
            }
        }
    }

    pub fn process_poison(&mut self, team_side: TeamSide, index: usize) -> BattleEventFeedbackEntry {
        let damage = calculate_poison_damage(self.get_vitality());
        let damage_taken = self.process_flat_damage(damage);
        self.on_poison_damage_taken(damage_taken);
        BattleEventFeedbackEntry {
            target_team: team_side,
            target_monster_index: index,
            feedback_type: BattleEventFeedbackType::PoisonDamageTaken,
            value: Some(damage_taken as i64),
            factor: None,
        }
    }

    pub fn process_status_effect(&mut self, status_effect: StatusEffect, team_side: TeamSide, index: usize) -> Vec<BattleEventFeedbackEntry> {
        let mut effect_feedback = match status_effect {
            StatusEffect::Poisoned => {
                let feedback = self.process_poison(team_side, index);
                vec![feedback]
            }
            _ => Vec::new(),
        };

        let effect_removed = self.tick_status_effect(status_effect);
        if effect_removed {
            effect_feedback.push(BattleEventFeedbackEntry {
                target_team: team_side,
                target_monster_index: index,
                feedback_type: status_effect.get_expired_feedback_type(),
                value: None,
                factor: None,
            })
        };
        effect_feedback
    }

    pub fn process_status_effects(&mut self, team_side: TeamSide, index: usize) -> Vec<BattleEventFeedbackEntry> {
        let effects: Vec<StatusEffect> = self.status_effects.keys().copied().collect();
        effects.into_iter()
            .flat_map(|effect| self.process_status_effect(effect, team_side, index))
            .collect()
    }

    pub fn process_try_act(&mut self, team_side: TeamSide, index: usize, action_index: usize) -> Option<Vec<BattleEventFeedbackEntry>> {
        if self.has_status_effect(StatusEffect::Paralyzed) {
            let cant_move = get_game_data().config.paralysis_stun_chance.roll();
            if cant_move {
                let feedback = BattleEventFeedbackEntry {
                    target_team: team_side,
                    target_monster_index: index,
                    feedback_type: BattleEventFeedbackType::ParalyzedCantAct,
                    value: None,
                    factor: None,
                };
                self.on_paralyzed_while_trying_to_act();
                return Some(vec![feedback]);
            }
        }
        None
    }

    pub fn on_turn_end(&mut self, team: TeamSide, index: usize) -> Vec<BattleEventFeedbackEntry> {
        let mut feedback_entries = Vec::new();

        let energy_generated = self.generate_energy(self.get_momentum());
        if energy_generated > 0 {
            feedback_entries.push(
                BattleEventFeedbackEntry {
                    target_team: team,
                    target_monster_index: index,
                    feedback_type: BattleEventFeedbackType::MomentumGeneratedEnergy,
                    value: Some(energy_generated as i64),
                    factor: None,
                }
            )
        };

        let status_effect_feedback = self.process_status_effects(team, index);
        feedback_entries.extend(status_effect_feedback);

        feedback_entries
    }

    pub fn on_action_used(&mut self, action_index: usize) -> Result<(), BattleError> {
        self.get_action_mut(action_index)
            .ok_or(BattleError::InvalidActionIndex)
            .map(|action| action.on_use())
    }

    pub fn on_momentum_used(&mut self, amount: u32) {
        self.stats.on_momentum_used(amount);
    }

    pub fn on_energy_used(&mut self, amount: u32) {
        self.stats.on_energy_used(amount);
    }

    pub fn on_hp_used(&mut self, amount: u32) {
        self.stats.on_hp_used(amount);
    }

    pub fn on_damage_dealt(&mut self, amount: u32) {
        self.stats.on_damage_dealt(amount);
    }

    pub fn on_damage_taken(&mut self, amount: u32) {
        self.stats.on_damage_taken(amount);
    }

    pub fn on_hp_heal_given(&mut self, amount: u32) {
        self.stats.on_hp_heal_given(amount);
    }

    pub fn on_hp_heal_received(&mut self, amount: u32) {
        self.stats.on_hp_heal_received(amount);
    }

    pub fn on_momentum_generated(&mut self, amount: u32) {
        self.stats.on_momentum_generated(amount);
    }

    pub fn on_energy_generated(&mut self, amount: u32) {
        self.stats.on_energy_generated(amount);
    }

    pub fn on_momentum_generated_for_others(&mut self, amount: u32) {
        self.stats.on_momentum_generated_for_others(amount);
    }

    pub fn on_energy_generated_for_others(&mut self, amount: u32) {
        self.stats.on_energy_generated_for_others(amount);
    }

    pub fn on_poison_damage_taken(&mut self, amount: u32) {
        self.stats.on_poison_damage_taken(amount);
    }

    pub fn on_paralyzed_while_trying_to_act(&mut self) {
        self.stats.on_paralyzed_while_trying_to_act();
    }

    pub fn on_poison_applied(&mut self, count: u32) {
        self.stats.on_poison_applied(count);
    }

    pub fn on_paralysis_applied(&mut self, count: u32) {
        self.stats.on_paralysis_applied(count);
    }

    pub fn on_poison_received(&mut self) {
        self.stats.on_poison_received();
    }

    pub fn on_paralysis_received(&mut self) {
        self.stats.on_paralysis_received();
    }
}

impl MonsterDataAccess for BattleMonster {
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

impl From<StoredMonster> for BattleMonster {
    fn from(stored_monster: StoredMonster) -> Self {
        Self::from_stored_monster(stored_monster)
    }
}
