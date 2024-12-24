use crate::battle_logic::battle_event_cost::BattleEventCost;
use crate::battle_logic::battle_event_type::BattleEventType;
use crate::enums::action_target::ActionTarget;
use crate::get_game_data;
use crate::serialization::arc_ref::ArcRefFromKey;
use crate::traits::action_data_access::ActionDataAccess;
use crate::traits::has_data_file::HasDataFileJson;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::utils::directories::action_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionData {
    #[serde(default)]
    id: u16,
    internal_name: String,
    event_types: Vec<BattleEventType>,
    potential_targets: Vec<ActionTarget>,
    priority: u8,
    #[serde(default)]
    costs: Vec<BattleEventCost>,
}

impl ArcRefFromKey for ActionData {
    type Key = u16;

    fn to_key(&self) -> Self::Key {
        self.get_id()
    }

    fn from_key(key: &u16) -> Option<Arc<Self>>
    where
        Self: Sized,
    {
        get_game_data().actions.get(*key).cloned()
    }
}

impl HasDataFileJson for ActionData {
    fn data_file_path() -> PathBuf {
        action_data_path()
    }
}

impl HasId for ActionData {
    type Id = u16;

    fn id(&self) -> u16 {
        self.get_id()
    }

    fn with_id(self, id: Self::Id) -> Self {
        Self { id, ..self }
    }
}

impl HasInternalName for ActionData {
    fn internal_name(&self) -> &str {
        self.get_internal_name()
    }
}

impl ActionDataAccess for ActionData {
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_internal_name(&self) -> &str {
        &self.internal_name
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

    fn get_potential_targets(&self) -> &[ActionTarget] {
        &self.potential_targets
    }

    fn has_potential_target(&self, action_target: &ActionTarget) -> bool {
        self.potential_targets.contains(action_target)
    }

    fn get_costs(&self) -> &[BattleEventCost] {
        &self.costs
    }
}