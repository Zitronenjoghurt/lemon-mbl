use crate::enums::damage_type::DamageType;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::enums::type_resonance::TypeResonance;
use crate::traits::has_data_file::HasDataFileJson;
use crate::traits::has_id::HasId;
use crate::utils::directories::damage_type_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhysicalTypeRelation {
    typing: MonsterPhysicalType,
    resonance: TypeResonance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElementalTypeRelation {
    typing: MonsterElementalType,
    resonance: TypeResonance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageTypeData {
    #[serde(default)]
    damage_type: DamageType,
    physical_relations: Vec<PhysicalTypeRelation>,
    elemental_relations: Vec<ElementalTypeRelation>,
}

impl DamageTypeData {
    pub fn get_physical_type_resonance(&self, physical_type: MonsterPhysicalType) -> Option<TypeResonance> {
        self.physical_relations.iter().find(|r| r.typing == physical_type).map(|r| r.resonance)
    }

    pub fn get_elemental_type_resonance(&self, elemental_type: MonsterElementalType) -> Option<TypeResonance> {
        self.elemental_relations.iter().find(|r| r.typing == elemental_type).map(|r| r.resonance)
    }

    pub fn calculate_damage_factor(
        &self,
        physical_types: &[MonsterPhysicalType],
        elemental_types: &[MonsterElementalType],
    ) -> f64 {
        let mut factor = 1.0;

        for physical_type in physical_types {
            if let Some(resonance) = self.get_physical_type_resonance(*physical_type) {
                factor *= resonance.get_damage_factor()
            }
        }

        for elemental_type in elemental_types {
            if let Some(resonance) = self.get_elemental_type_resonance(*elemental_type) {
                factor *= resonance.get_damage_factor()
            }
        }

        factor
    }
}

impl HasId for DamageTypeData {
    type Id = DamageType;

    fn id(&self) -> Self::Id {
        self.damage_type
    }

    fn with_id(self, id: Self::Id) -> Self {
        Self { damage_type: id, ..self }
    }
}

impl HasDataFileJson for DamageTypeData {
    fn data_file_path() -> PathBuf {
        damage_type_data_path()
    }
}