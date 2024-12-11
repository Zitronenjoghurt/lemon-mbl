use crate::battle_logic::battle_event_type::BattleEventType;
use crate::entities::action_data::ActionData;
use crate::enums::action_target::ActionTarget;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::action_data_access::ActionDataAccess;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredAction {
    #[serde(with = "arc_ref")]
    data: Arc<ActionData>,
    total_use_count: u64,
}

impl StoredAction {
    pub fn create(id: u16) -> Option<Self> {
        get_game_data().actions.get(id)
            .map(|data| Self::from_data(Arc::clone(data)))
    }

    pub fn from_data(data: Arc<ActionData>) -> Self {
        Self {
            data,
            total_use_count: 0,
        }
    }

    pub fn on_use(&mut self) {
        self.total_use_count += 1;
    }
}

impl ActionDataAccess for StoredAction {
    fn get_id(&self) -> u16 {
        self.data.get_id()
    }

    fn get_internal_name(&self) -> &str {
        self.data.get_internal_name()
    }

    fn get_event_types(&self) -> &[BattleEventType] {
        self.data.get_event_types()
    }

    fn has_event_type(&self, event_type: &BattleEventType) -> bool {
        self.data.has_event_type(event_type)
    }

    fn get_potential_targets(&self) -> &[ActionTarget] {
        self.data.get_potential_targets()
    }

    fn has_potential_target(&self, action_target: &ActionTarget) -> bool {
        self.data.has_potential_target(action_target)
    }
}