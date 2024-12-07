use crate::data_structures::entity_library::EntityLibrary;
use crate::entities::monster::Monster;
use crate::enums::monster_flags::MonsterFlag;

#[test]
fn test_monster_library() {
    let monster_library: EntityLibrary<Monster> = EntityLibrary::from_yaml().unwrap();
    let test_monster = monster_library.get(0).unwrap().clone();

    assert_eq!(test_monster.hp(), 10);
    assert_eq!(test_monster.attack(), 12);
    assert_eq!(test_monster.defense(), 15);
    assert_eq!(test_monster.flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying))
}