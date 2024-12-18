use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonsterStats {
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub hp_heal_given: u32,
    pub hp_heal_received: u32,
    pub momentum_used: u32,
    pub energy_used: u32,
    pub hp_used: u32,
    pub momentum_generated: u32,
    pub energy_generated: u32,
    pub momentum_generated_for_others: u32,
    pub energy_generated_for_others: u32,
    pub poison_damage_taken: u32,
    pub times_paralyzed_while_trying_to_act: u32,
    pub times_poison_applied: u32,
    pub times_paralysis_applied: u32,
    pub times_poison_received: u32,
    pub times_paralysis_received: u32,
}

impl Add for MonsterStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            damage_dealt: self.damage_dealt.saturating_add(other.damage_dealt),
            damage_taken: self.damage_taken.saturating_add(other.damage_taken),
            hp_heal_given: self.hp_heal_given.saturating_add(other.hp_heal_given),
            hp_heal_received: self.hp_heal_received.saturating_add(other.hp_heal_received),
            momentum_used: self.momentum_used.saturating_add(other.momentum_used),
            energy_used: self.energy_used.saturating_add(other.energy_used),
            hp_used: self.hp_used.saturating_add(other.hp_used),
            momentum_generated: self.momentum_generated.saturating_add(other.momentum_generated),
            energy_generated: self.energy_generated.saturating_add(other.energy_generated),
            momentum_generated_for_others: self.momentum_generated_for_others.saturating_add(other.momentum_generated_for_others),
            energy_generated_for_others: self.energy_generated_for_others.saturating_add(other.energy_generated_for_others),
            poison_damage_taken: self.poison_damage_taken.saturating_add(other.poison_damage_taken),
            times_paralyzed_while_trying_to_act: self.times_paralyzed_while_trying_to_act.saturating_add(other.times_paralyzed_while_trying_to_act),
            times_poison_applied: self.times_poison_applied.saturating_add(other.times_poison_applied),
            times_paralysis_applied: self.times_paralysis_applied.saturating_add(other.times_paralysis_applied),
            times_poison_received: self.times_poison_received.saturating_add(other.times_poison_received),
            times_paralysis_received: self.times_paralysis_received.saturating_add(other.times_paralysis_received),
        }
    }
}

impl MonsterStats {
    pub fn on_damage_dealt(&mut self, amount: u32) {
        self.damage_dealt = self.damage_dealt.saturating_add(amount);
    }

    pub fn on_damage_taken(&mut self, amount: u32) {
        self.damage_taken = self.damage_taken.saturating_add(amount);
    }

    pub fn on_hp_heal_given(&mut self, amount: u32) {
        self.hp_heal_given = self.hp_heal_given.saturating_add(amount);
    }

    pub fn on_hp_heal_received(&mut self, amount: u32) {
        self.hp_heal_received = self.hp_heal_received.saturating_add(amount);
    }

    pub fn on_momentum_used(&mut self, amount: u32) {
        self.momentum_used = self.momentum_used.saturating_add(amount);
    }

    pub fn on_energy_used(&mut self, amount: u32) {
        self.energy_used = self.energy_used.saturating_add(amount);
    }

    pub fn on_hp_used(&mut self, amount: u32) {
        self.hp_used = self.hp_used.saturating_add(amount);
    }

    pub fn on_momentum_generated(&mut self, amount: u32) {
        self.momentum_generated = self.momentum_generated.saturating_add(amount);
    }

    pub fn on_energy_generated(&mut self, amount: u32) {
        self.energy_generated = self.energy_generated.saturating_add(amount);
    }

    pub fn on_momentum_generated_for_others(&mut self, amount: u32) {
        self.momentum_generated_for_others = self.momentum_generated_for_others.saturating_add(amount);
    }

    pub fn on_energy_generated_for_others(&mut self, amount: u32) {
        self.energy_generated_for_others = self.energy_generated_for_others.saturating_add(amount);
    }

    pub fn on_poison_damage_taken(&mut self, amount: u32) {
        self.poison_damage_taken = self.poison_damage_taken.saturating_add(amount);
    }

    pub fn on_paralyzed_while_trying_to_act(&mut self) {
        self.times_paralyzed_while_trying_to_act = self.times_paralyzed_while_trying_to_act.saturating_add(1);
    }

    pub fn on_poison_applied(&mut self, count: u32) {
        self.times_poison_applied = self.times_poison_applied.saturating_add(count);
    }

    pub fn on_paralysis_applied(&mut self, count: u32) {
        self.times_paralysis_applied = self.times_paralysis_applied.saturating_add(count);
    }

    pub fn on_poison_received(&mut self) {
        self.times_poison_received = self.times_poison_received.saturating_add(1);
    }

    pub fn on_paralysis_received(&mut self) {
        self.times_paralysis_received = self.times_paralysis_received.saturating_add(1);
    }
}