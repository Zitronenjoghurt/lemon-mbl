use crate::enums::damage_type::DamageType;
use crate::get_game_data;
use crate::traits::action_data_access::ActionDataAccess;

#[test]
fn test_data_integrity() {
    let game_data = get_game_data();
    let actions_data_library = &game_data.actions;
    let test_action = actions_data_library.get(0).unwrap();

    assert_eq!(test_action.get_id(), 0);
    assert_eq!(test_action.get_internal_name(), "test_action");
    assert_eq!(test_action.get_damage_types().len(), 2);
    assert!(test_action.has_damage_type(DamageType::Bludgeoning));
    assert!(test_action.has_damage_type(DamageType::Fire));
}