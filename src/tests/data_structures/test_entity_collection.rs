use crate::data_structures::entity_collection::EntityCollection;
use crate::entities::stored_monster::StoredMonster;
use crate::traits::has_assignable_id::HasAssignableId;

#[test]
fn test_crud() {
    let collection: EntityCollection<StoredMonster> = EntityCollection::new();
    let mut test_monster = StoredMonster::create(0).unwrap();
    let storage_id = collection.insert(test_monster.clone());
    assert_eq!(storage_id, 1);

    test_monster.set_id(1);
    let monster = collection.get(1).unwrap();
    assert_eq!(test_monster, monster);

    // Change storage ID of a stored monster which should not be done, it's just for testing purposes
    collection.update(1, |monster| monster.set_id(2));
    let monster = collection.get(1).unwrap();
    assert_eq!(monster.get_id(), 2);

    collection.remove(1);
    let monster = collection.get(1);
    assert!(monster.is_none());
}