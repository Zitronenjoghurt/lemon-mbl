use crate::utils::directories::config_data_path;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ConfigData {
    #[serde(default = "default_maximum_damage_factor")]
    pub maximum_damage_factor: f64,
}

fn default_maximum_damage_factor() -> f64 {
    16.0
}

impl ConfigData {
    pub fn from_yaml() -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_file_path = config_data_path();
        let contents = fs::read_to_string(yaml_file_path)?;
        let config: ConfigData = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}