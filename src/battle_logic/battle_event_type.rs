use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_state::BattleState;
use crate::battle_logic::events::damage_event::DamageEventType;
use crate::battle_logic::events::heal_event::HealEventType;
use crate::enums::damage_type::DamageType;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BattleEventType {
    Damage(DamageEventType) = 0,
    Heal(HealEventType) = 1,
}

impl BattleEventType {
    pub fn get_identifiers() -> Vec<String> {
        vec!["Damage".to_string(), "Heal".to_string()]
    }

    pub fn process(&self, state: &mut BattleState, source_team: TeamSide, target_team: TeamSide, source_index: usize, target_index: usize) -> Result<(), BattleError> {
        match self {
            BattleEventType::Damage(damage_event_type) => { damage_event_type.process(state, source_team, target_team, source_index, target_index) }
            BattleEventType::Heal(heal_event_type) => { heal_event_type.process(state, source_team, target_team, source_index, target_index) }
        }
    }

    pub fn get_amount(&self) -> Option<u16> {
        match self {
            BattleEventType::Damage(event) => Some(event.amount),
            BattleEventType::Heal(event) => Some(event.amount),
        }
    }

    pub fn get_damage_types(&self) -> Option<&Vec<DamageType>> {
        match self {
            BattleEventType::Damage(event) => Some(&event.damage_types),
            _ => None,
        }
    }

    pub fn has_damage_type(&self, damage_type: DamageType) -> bool {
        self.get_damage_types().is_some_and(|damage_types| damage_types.contains(&damage_type))
    }
}