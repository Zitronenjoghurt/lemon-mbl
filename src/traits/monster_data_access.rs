use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;

pub trait MonsterDataAccess {
    fn get_id(&self) -> u16;
    fn get_internal_name(&self) -> &str;
    fn get_vitality(&self) -> u16;
    fn get_potential(&self) -> u16;
    fn get_control(&self) -> u16;
    fn get_strength(&self) -> u16;
    fn get_resilience(&self) -> u16;
    fn get_speed(&self) -> u16;
    fn get_technique(&self) -> u16;
    fn get_agility(&self) -> u16;
    fn get_vigilance(&self) -> u16;
    fn get_focus(&self) -> u16;
    fn get_flags(&self) -> &[MonsterFlag];
    fn has_flag(&self, flag: MonsterFlag) -> bool;
    fn get_physical_types(&self) -> &[MonsterPhysicalType];
    fn has_physical_type(&self, physical_type: MonsterPhysicalType) -> bool;
    fn get_elemental_types(&self) -> &[MonsterElementalType];
    fn has_elemental_type(&self, elemental_type: MonsterElementalType) -> bool;
}