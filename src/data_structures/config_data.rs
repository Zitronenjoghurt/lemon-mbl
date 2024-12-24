use crate::calculations::int_bps::IntBps;
use crate::utils::directories::config_data_path;
use serde::{Deserialize, Serialize};
use std::fs;

#[cfg_attr(feature = "dev", derive(Clone))]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ConfigData {
    #[serde(default = "default_maximum_damage_factor")]
    pub maximum_damage_factor: f64,
    /// Percentage of the monsters max HP lost every turn (normalized)
    #[serde(default = "default_poison_damage_factor")]
    pub poison_damage_per_turn: f64,
    #[serde(default = "default_paralysis_stun_chance")]
    pub paralysis_stun_chance: IntBps,
}

fn default_maximum_damage_factor() -> f64 {
    16.0
}

fn default_poison_damage_factor() -> f64 {
    0.1
}

fn default_paralysis_stun_chance() -> IntBps {
    IntBps(5000)
}

impl ConfigData {
    pub fn from_yaml() -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_file_path = config_data_path();
        let contents = fs::read_to_string(yaml_file_path)?;
        let config: ConfigData = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}