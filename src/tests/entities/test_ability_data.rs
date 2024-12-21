use crate::enums::ability_target::AbilityTarget;
use crate::enums::ability_trigger_type::AbilityTriggerType;
use crate::enums::event_target::EventTarget;
use crate::enums::resource_type::ResourceType;
use crate::get_game_data;
use crate::traits::ability_data_access::AbilityDataAccess;

#[test]
fn test_data_integrity() {
    let game_data = get_game_data();
    let abilities_data_library = &game_data.abilities;
    let full_focus = abilities_data_library.get(0).unwrap();

    assert_eq!(full_focus.get_id(), 0);
    assert_eq!(full_focus.get_internal_name(), "full_focus");
    assert_eq!(full_focus.get_event_types().len(), 1);
    assert_eq!(full_focus.get_priority(), 250);
    assert_eq!(full_focus.get_target(), AbilityTarget::OneSelf);
    assert_eq!(full_focus.get_trigger(), AbilityTriggerType::EveryTurn);

    let event_type = &full_focus.get_event_types()[0];
    assert_eq!(event_type.get_amount().unwrap(), 5);
    assert_eq!(event_type.get_resource_type().unwrap(), ResourceType::Momentum);
    assert_eq!(event_type.get_target().unwrap(), EventTarget::SourceMonster);
}
