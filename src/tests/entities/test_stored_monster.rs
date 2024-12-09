use crate::entities::battle_monster::BattleMonster;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_flag::MonsterFlag;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::traits::monster_data_access::MonsterDataAccess;

#[test]
fn test_creation() {
    let test_monster = StoredMonster::create(0).unwrap();
    assert_eq!(test_monster.get_storage_id(), 0);
    assert_eq!(test_monster.get_current_hp(), 50);

    assert_eq!(test_monster.get_id(), 0);
    assert_eq!(test_monster.get_internal_name(), "test_monster");
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

#[test]
fn test_to_from_battle_monster() {
    let test_monster = StoredMonster::create(0).unwrap();
    let battle_monster = BattleMonster::from(test_monster.clone());
    let test_monster_again = StoredMonster::from(battle_monster);
    assert_eq!(test_monster, test_monster_again);
}