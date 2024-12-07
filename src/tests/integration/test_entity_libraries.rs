use crate::get_game_data;
use lemon_mbl_game_data::enums::monster_flags::MonsterFlag;
use lemon_mbl_game_data::traits::has_internal_name::HasInternalName;

#[test]
fn test_monster_data_library() {
    let game_data = get_game_data();
    let monster_data_library = &game_data.monsters;
    let test_monster = monster_data_library.get(0).unwrap();

    assert_eq!(test_monster.internal_name(), "test_monster");
    assert_eq!(test_monster.hp(), 10);
    assert_eq!(test_monster.attack(), 12);
    assert_eq!(test_monster.defense(), 15);
    assert_eq!(test_monster.flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying))
}