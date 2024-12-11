use crate::enums::damage_type::DamageType;
use crate::get_game_data;
use crate::traits::action_data_access::ActionDataAccess;

#[test]
fn test_data_integrity() {
    let game_data = get_game_data();
    let actions_data_library = &game_data.actions;
    let test_action = actions_data_library.get(0).unwrap();

    assert_eq!(test_action.get_id(), 0);
    assert_eq!(test_action.get_internal_name(), "test_attack");
    assert_eq!(test_action.get_event_types().len(), 1);

    let damage_event_type = &test_action.get_event_types()[0];
    assert_eq!(damage_event_type.get_amount().unwrap(), 10);
    assert!(damage_event_type.has_damage_type(DamageType::Piercing));
}
