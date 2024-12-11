use crate::battle_logic::battle_event_type::BattleEventType;
use crate::enums::action_target::ActionTarget;

pub trait ActionDataAccess {
    fn get_id(&self) -> u16;
    fn get_internal_name(&self) -> &str;
    fn get_priority(&self) -> u8;
    fn get_event_types(&self) -> &[BattleEventType];
    fn has_event_type(&self, event_type: &BattleEventType) -> bool;
    fn get_potential_targets(&self) -> &[ActionTarget];
    fn has_potential_target(&self, action_target: &ActionTarget) -> bool;
}