use crate::game_data::data_objects::monster_data::MonsterData;
use crate::serialization::arc_ref;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMonster {
    #[serde(with = "arc_ref")]
    data: Arc<MonsterData>,
}