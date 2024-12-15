use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::battle_logic::events::damage_event::DamageEventType;
use crate::battle_logic::events::generate_resource_event::GenerateResourceEventType;
use crate::battle_logic::events::heal_event::HealEventType;
use crate::enums::damage_type::DamageType;
use crate::enums::event_target::EventTarget;
use crate::enums::resource_type::ResourceType;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BattleEventType {
    Damage(DamageEventType) = 0,
    Heal(HealEventType) = 1,
    GenerateResource(GenerateResourceEventType) = 2,
}

impl BattleEventType {
    pub fn get_identifiers() -> Vec<String> {
        vec![
            "Damage".to_string(),
            "Heal".to_string(),
            "GenerateResource".to_string(),
        ]
    }

    pub fn process(
        &self,
        state: &mut BattleState,
        source_team: TeamSide,
        target_team: TeamSide,
        source_index: usize,
        target_index: usize,
    ) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        match self {
            Self::Damage(damage_event_type) => { damage_event_type.process(state, source_team, target_team, source_index, target_index) }
            Self::Heal(heal_event_type) => { heal_event_type.process(state, source_team, target_team, source_index, target_index) }
            Self::GenerateResource(resource_event_type) => { resource_event_type.process(state, source_team, target_team, source_index, target_index) }
        }
    }

    pub fn get_amount(&self) -> Option<u32> {
        match self {
            Self::Damage(event) => Some(event.amount),
            Self::Heal(event) => Some(event.amount),
            Self::GenerateResource(event) => Some(event.amount),
        }
    }

    pub fn get_target(&self) -> Option<EventTarget> {
        match self {
            Self::Damage(event) => Some(event.target),
            Self::Heal(event) => Some(event.target),
            Self::GenerateResource(event) => Some(event.target),
        }
    }

    pub fn get_damage_types(&self) -> Option<&Vec<DamageType>> {
        match self {
            Self::Damage(event) => Some(&event.damage_types),
            _ => None,
        }
    }

    pub fn has_damage_type(&self, damage_type: DamageType) -> bool {
        self.get_damage_types().is_some_and(|damage_types| damage_types.contains(&damage_type))
    }

    pub fn get_resource_type(&self) -> Option<ResourceType> {
        match self {
            Self::GenerateResource(event) => Some(event.resource),
            _ => None,
        }
    }
}