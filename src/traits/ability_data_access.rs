use crate::battle_logic::battle_event_type::BattleEventType;
use crate::enums::ability_target::AbilityTarget;
use crate::enums::ability_trigger_type::AbilityTriggerType;

pub trait AbilityDataAccess {
    fn get_id(&self) -> u16;
    fn get_internal_name(&self) -> &str;
    fn get_priority(&self) -> u8;
    fn get_event_types(&self) -> &[BattleEventType];
    fn has_event_type(&self, event_type: &BattleEventType) -> bool;
    fn get_target(&self) -> AbilityTarget;
    fn get_trigger(&self) -> AbilityTriggerType;
}