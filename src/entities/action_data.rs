use crate::battle_logic::battle_event_type::BattleEventType;
use crate::enums::action_target::ActionTarget;
use crate::get_game_data;
use crate::serialization::arc_ref::ArcRefFromKey;
use crate::traits::action_data_access::ActionDataAccess;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::utils::directories::action_data_path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionData {
    id: u16,
    #[serde(default)]
    internal_name: String,
    event_types: Vec<BattleEventType>,
    potential_targets: Vec<ActionTarget>,
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

    // The yaml de/serializer expects a yaml tag for the nested BattleEventType enum
    // Problem is: JSON schemas don't support yaml tags, which makes it all the harder to edit the files
    // That's why we turn the object keys into yaml tags in the preprocessing step
    fn preprocess(contents: String) -> String {
        let event_types = BattleEventType::get_identifiers();
        let mut processed = contents;

        for event_type in event_types {
            let pattern = format!(r"(?m)^(\s*)(- )?({}: *)", event_type);
            let regex = Regex::new(&pattern).unwrap();
            processed = regex.replace_all(&processed, format!("$1$2!{}", event_type)).to_string();
        }

        processed
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
}