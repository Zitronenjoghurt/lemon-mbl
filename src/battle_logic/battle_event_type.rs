use crate::battle_logic::battle_error::BattleError;
use crate::battle_logic::battle_event_feedback::BattleEventFeedbackEntry;
use crate::battle_logic::battle_state::BattleState;
use crate::battle_logic::events::apply_status_effect_event::ApplyStatusEffectEventType;
use crate::battle_logic::events::damage_event::DamageEventType;
use crate::battle_logic::events::generate_resource_event::GenerateResourceEventType;
use crate::battle_logic::events::heal_event::HealEventType;
use crate::enums::damage_type::DamageType;
use crate::enums::event_target::EventTarget;
use crate::enums::resource_type::ResourceType;
use crate::enums::team_side::TeamSide;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BattleEventType {
    Damage(DamageEventType) = 0,
    Heal(HealEventType) = 1,
    GenerateResource(GenerateResourceEventType) = 2,
    ApplyStatusEffect(ApplyStatusEffectEventType) = 3,
}

impl BattleEventType {
    pub fn get_identifiers() -> Vec<String> {
        vec![
            "Damage".to_string(),
            "Heal".to_string(),
            "GenerateResource".to_string(),
            "ApplyStatusEffect".to_string(),
        ]
    }

    pub fn process(
        &mut self,
        state: &mut BattleState,
        source_team: TeamSide,
        target_team: TeamSide,
        source_index: usize,
        target_index: usize,
    ) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        match self {
            Self::Damage(event) => { event.process(state, source_team, target_team, source_index, target_index) }
            Self::Heal(event) => { event.process(state, source_team, target_team, source_index, target_index) }
            Self::GenerateResource(event) => { event.process(state, source_team, target_team, source_index, target_index) }
            Self::ApplyStatusEffect(event) => { event.process(state, source_team, target_team, source_index, target_index) }
        }
    }

    pub fn get_amount(&self) -> Option<u32> {
        match self {
            Self::Damage(event) => Some(event.amount),
            Self::Heal(event) => Some(event.amount),
            Self::GenerateResource(event) => Some(event.amount),
            _ => None,
        }
    }

    pub fn get_target(&self) -> Option<EventTarget> {
        match self {
            Self::Damage(event) => Some(event.target),
            Self::Heal(event) => Some(event.target),
            Self::GenerateResource(event) => Some(event.target),
            Self::ApplyStatusEffect(event) => Some(event.target),
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

    pub fn on_start(&mut self) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        Ok(Vec::new())
    }

    pub fn start_triggered(&self) -> bool {
        true
    }

    pub fn should_remove(&self) -> bool {
        true
    }

    pub fn on_end(&mut self) -> Result<Vec<BattleEventFeedbackEntry>, BattleError> {
        Ok(Vec::new())
    }
}

impl Serialize for BattleEventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;

        match self {
            BattleEventType::Damage(event) => {
                map.serialize_entry("Damage", event)?;
            }
            BattleEventType::Heal(event) => {
                map.serialize_entry("Heal", event)?;
            }
            BattleEventType::GenerateResource(event) => {
                map.serialize_entry("GenerateResource", event)?;
            }
            BattleEventType::ApplyStatusEffect(event) => {
                map.serialize_entry("ApplyStatusEffect", event)?;
            }
        }
        map.end()
    }
}