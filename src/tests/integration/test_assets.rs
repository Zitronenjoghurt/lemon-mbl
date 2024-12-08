use crate::get_game_data;

#[test]
fn test_monster_image_bundling() {
    let game_data = get_game_data();
    let existing_image_ids = game_data.monster_images.available_ids();
    assert!(existing_image_ids.contains(&0));
}