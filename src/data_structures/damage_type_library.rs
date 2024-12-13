use crate::data_structures::data_library::DataLibrary;
use crate::entities::damage_type_data::DamageTypeData;
use crate::enums::damage_type::DamageType;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use serde::{Deserialize, Serialize};

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
        damage_type: &DamageType,
        physical_types: &Vec<MonsterPhysicalType>,
        elemental_types: &Vec<MonsterElementalType>,
    ) -> f64 {
        self.data.get(*damage_type).map_or(1.0, |damage_type_data| {
            damage_type_data.calculate_damage_factor(
                physical_types,
                elemental_types,
            )
        })
    }
}