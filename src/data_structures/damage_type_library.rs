use crate::data_structures::data_library::DataLibrary;
use crate::entities::damage_type_data::DamageTypeData;
use crate::enums::damage_type::DamageType;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "dev", derive(Clone))]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DamageTypeLibrary {
    data: DataLibrary<DamageTypeData>,
}

impl DamageTypeLibrary {
    pub fn from_yaml() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { data: DataLibrary::from_yaml()? })
    }

    pub fn calculate_damage_factor(
        &self,
        damage_types: &[DamageType],
        physical_types: &[MonsterPhysicalType],
        elemental_types: &[MonsterElementalType],
    ) -> f64 {
        damage_types.iter().fold(1.0_f64, |acc: f64, damage_type| {
            let factor = self.data.get(*damage_type).map_or(1.0, |damage_type_data| {
                damage_type_data.calculate_damage_factor(
                    physical_types,
                    elemental_types,
                )
            });

            if acc.is_infinite() || factor.is_infinite() {
                f64::INFINITY
            } else if acc.is_nan() || factor.is_nan() {
                1.0_f64
            } else {
                acc * factor
            }
        }).clamp(0.0, get_game_data().config.maximum_damage_factor)
    }
}