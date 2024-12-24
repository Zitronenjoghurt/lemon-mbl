use crate::battle_logic::battle_event_type::BattleEventType;
use crate::enums::ability_target::AbilityTarget;
use crate::enums::ability_trigger_type::AbilityTriggerType;
use crate::traits::ability_data_access::AbilityDataAccess;
use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use crate::utils::directories::ability_data_path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityData {
    id: u16,
    #[serde(default)]
    internal_name: String,
    event_types: Vec<BattleEventType>,
    target: AbilityTarget,
    trigger: AbilityTriggerType,
    priority: u8,
}

impl HasDataFileYaml for AbilityData {
    fn data_file_path() -> PathBuf {
        ability_data_path()
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

    #[cfg(feature = "dev")]
    fn postprocess(contents: String) -> String {
        let internal_name_pattern = r"(?m)^ {2}internal_name:.*\n";
        let regex = Regex::new(internal_name_pattern).unwrap();
        regex.replace_all(&contents, "").to_string()
    }
}

impl HasId for AbilityData {
    type Id = u16;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl HasInternalName for AbilityData {
    fn internal_name(&self) -> &str {
        &self.internal_name
    }

    fn with_internal_name(self, name: String) -> Self {
        Self { internal_name: name, ..self }
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