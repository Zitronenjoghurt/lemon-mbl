use crate::data_structures::entity_library::EntityLibrary;
use crate::directories::monster_data_path;
use crate::entities::monster::Monster;

#[test]
fn test_monster_library() {
    let monsters_path = monster_data_path();
    let monster_library: EntityLibrary<Monster> = EntityLibrary::from_yaml(monsters_path).unwrap();
    let test_monster = monster_library.get(0).unwrap().clone();

    assert_eq!(test_monster.hp(), 10);
    assert_eq!(test_monster.attack(), 12);
    assert_eq!(test_monster.defense(), 15);
}