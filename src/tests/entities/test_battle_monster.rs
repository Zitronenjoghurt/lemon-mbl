use crate::entities::battle_monster::BattleMonster;
use lemon_mbl_game_data::enums::monster_flags::MonsterFlag;

#[test]
pub fn test_creation() {
    let test_monster = BattleMonster::create(0).unwrap();
    assert_eq!(test_monster.get_current_hp(), 10);
    assert_eq!(test_monster.get_id(), 0);
    assert_eq!(test_monster.get_internal_name(), "test_monster");
    assert_eq!(test_monster.get_max_hp(), 10);
    assert_eq!(test_monster.get_attack(), 12);
    assert_eq!(test_monster.get_defense(), 15);
    assert_eq!(test_monster.get_flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying));
}