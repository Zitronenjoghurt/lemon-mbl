use crate::data_structures::entity_collection::EntityCollection;
use crate::entities::stored_monster::StoredMonster;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    stored_monsters: EntityCollection<StoredMonster>,
}