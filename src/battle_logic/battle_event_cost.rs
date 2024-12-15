use crate::enums::resource_type::ResourceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BattleEventCost {
    pub resource: ResourceType,
    pub amount: u32,
}