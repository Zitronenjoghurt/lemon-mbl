use crate::data_structures::entity_library::EntityLibrary;
use crate::entities::monster_data::MonsterData;
use crate::enums::monster_flags::MonsterFlag;
use crate::traits::has_internal_name::HasInternalName;

#[test]
fn test_monster_data_library() {
    let monster_data_library: EntityLibrary<MonsterData> = EntityLibrary::from_yaml().unwrap();
    let test_monster = monster_data_library.get(0).unwrap().clone();

    assert_eq!(test_monster.internal_name(), "test_monster");
    assert_eq!(test_monster.hp(), 10);
    assert_eq!(test_monster.attack(), 12);
    assert_eq!(test_monster.defense(), 15);
    assert_eq!(test_monster.flags().len(), 1);
    assert!(test_monster.has_flag(MonsterFlag::Flying))
}