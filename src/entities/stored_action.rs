use crate::entities::action_data::ActionData;
use crate::get_game_data;
use crate::serialization::arc_ref;
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