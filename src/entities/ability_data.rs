use crate::battle_logic::battle_event_type::BattleEventType;
use crate::enums::ability_target::AbilityTarget;
use crate::enums::ability_trigger_type::AbilityTriggerType;
use crate::traits::ability_data_access::AbilityDataAccess;
use crate::traits::has_data_file::HasDataFileJson;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::utils::directories::ability_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityData {
    #[serde(default)]
    id: u16,
    internal_name: String,
    event_types: Vec<BattleEventType>,
    target: AbilityTarget,
    trigger: AbilityTriggerType,
    priority: u8,
}

impl HasDataFileJson for AbilityData {
    fn data_file_path() -> PathBuf {
        ability_data_path()
    }
}

impl HasId for AbilityData {
    type Id = u16;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn with_id(self, id: Self::Id) -> Self {
        Self { id, ..self }
    }
}

impl HasInternalName for AbilityData {
    fn internal_name(&self) -> &str {
        &self.internal_name
    }
}

impl AbilityDataAccess for AbilityData {
    fn get_id(&self) -> u16 {
        self.id()
    }

    fn get_internal_name(&self) -> &str {
        self.internal_name()
    }

    fn get_priority(&self) -> u8 {
        self.priority
    }

    fn get_event_types(&self) -> &[BattleEventType] {
        &self.event_types
    }

    fn has_event_type(&self, event_type: &BattleEventType) -> bool {
        self.event_types.contains(event_type)
    }

    fn get_target(&self) -> AbilityTarget {
        self.target
    }

    fn get_trigger(&self) -> AbilityTriggerType {
        self.trigger
    }
}