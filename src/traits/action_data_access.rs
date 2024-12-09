use crate::enums::damage_type::DamageType;

pub trait ActionDataAccess {
    fn get_id(&self) -> u16;
    fn get_internal_name(&self) -> &str;
    fn get_damage_types(&self) -> &[DamageType];
    fn has_damage_type(&self, damage_type: DamageType) -> bool;
}