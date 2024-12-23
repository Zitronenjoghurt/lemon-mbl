use crate::enums::locale::Locale;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;
use crate::traits::monster_data_access::MonsterDataAccess;

#[test]
fn test_data_integrity() {
    let game_data = get_game_data();
    let monster_data_library = &game_data.monsters;
    let test_monster = monster_data_library.get(0).unwrap();

    assert_eq!(test_monster.get_id(), 0);
    assert_eq!(test_monster.get_internal_name(), "test_monster");
    assert_eq!(game_data.i18n.get_monster_name(&Locale::English, 0).unwrap(), "Test Monster");
    assert_eq!(game_data.i18n.get_monster_description(&Locale::English, 0).unwrap(), "Its origins remain a mystery...");
    assert_eq!(test_monster.get_vitality(), 50);
    assert_eq!(test_monster.get_potential(), 60);
    assert_eq!(test_monster.get_control(), 10);
    assert_eq!(test_monster.get_strength(), 13);
    assert_eq!(test_monster.get_resilience(), 14);
    assert_eq!(test_monster.get_speed(), 15);
    assert_eq!(test_monster.get_technique(), 16);
    assert_eq!(test_monster.get_agility(), 17);
    assert_eq!(test_monster.get_vigilance(), 6000);
    assert_eq!(test_monster.get_focus(), 19);
    assert_eq!(test_monster.get_flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying));
    assert_eq!(test_monster.get_physical_types().len(), 1);
    assert!(test_monster.has_physical_type(MonsterPhysicalType::Construct));
    assert_eq!(test_monster.get_elemental_types().len(), 2);
    assert!(test_monster.has_elemental_type(MonsterElementalType::Force));
    assert!(test_monster.has_elemental_type(MonsterElementalType::Light));
}