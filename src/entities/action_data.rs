use crate::enums::damage_type::DamageType;
use crate::get_game_data;
use crate::serialization::arc_ref::ArcRefFromKey;
use crate::traits::action_data_access::ActionDataAccess;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::utils::directories::action_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionData {
    id: u16,
    #[serde(default)]
    internal_name: String,
    damage_types: Vec<DamageType>,
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

impl HasDataFileYaml for ActionData {
    fn data_file_path() -> PathBuf {
        action_data_path()
    }
}

impl HasId for ActionData {
    type Id = u16;

    fn id(&self) -> u16 {
        self.get_id()
    }
}

impl HasInternalName for ActionData {
    fn internal_name(&self) -> &str {
        self.get_internal_name()
    }

    fn with_internal_name(self, name: String) -> Self {
        Self {
            internal_name: name,
            ..self
        }
    }
}

impl ActionDataAccess for ActionData {
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_internal_name(&self) -> &str {
        &self.internal_name
    }

    fn get_damage_types(&self) -> &[DamageType] {
        &self.damage_types
    }

    fn has_damage_type(&self, damage_type: DamageType) -> bool {
        self.damage_types.contains(&damage_type)
    }
}