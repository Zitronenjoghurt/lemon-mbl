use crate::enums::monster_flags::MonsterFlag;
use crate::get_game_data;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;

#[test]
fn test_monster_data_library() {
    let game_data = get_game_data();
    let monster_data_library = &game_data.monsters;
    let test_monster = monster_data_library.get(0).unwrap();

    assert_eq!(test_monster.internal_name(), "test_monster");
    assert_eq!(test_monster.id(), 0);
    assert_eq!(test_monster.vitality(), 50);
    assert_eq!(test_monster.potential(), 60);
    assert_eq!(test_monster.control(), 10);
    assert_eq!(test_monster.strength(), 13);
    assert_eq!(test_monster.resilience(), 14);
    assert_eq!(test_monster.speed(), 15);
    assert_eq!(test_monster.technique(), 16);
    assert_eq!(test_monster.agility(), 17);
    assert_eq!(test_monster.vigilance(), 6000);
    assert_eq!(test_monster.focus(), 19);
    assert_eq!(test_monster.flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying))
}