use crate::entities::battle_monster::BattleMonster;
use crate::enums::monster_flags::MonsterFlag;

#[test]
fn test_creation() {
    let test_monster = BattleMonster::create(0).unwrap();
    assert_eq!(test_monster.get_id(), 0);
    assert_eq!(test_monster.get_internal_name(), "test_monster");
    assert_eq!(test_monster.get_max_hp(), 50);
    assert_eq!(test_monster.get_potential(), 60);
    assert_eq!(test_monster.get_control(), 10);
    assert_eq!(test_monster.get_strength(), 13);
    assert_eq!(test_monster.get_resilience(), 14);
    assert_eq!(test_monster.get_speed(), 15);
    assert_eq!(test_monster.get_technique(), 16);
    assert_eq!(test_monster.get_agility(), 17);
    assert_eq!(test_monster.get_vigilance(), 6000);
    assert_eq!(test_monster.get_focus(), 19);
    assert_eq!(test_monster.get_current_hp(), 50);
    assert_eq!(test_monster.get_desperation(), 0);
    assert_eq!(test_monster.get_momentum(), 0);
    assert_eq!(test_monster.get_energy(), 5);
    assert_eq!(test_monster.get_flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying));
}