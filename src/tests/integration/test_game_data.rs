use crate::get_game_data;
use lemon_mbl_game_data::game_data::GameData;
use std::sync::Arc;

#[test]
fn test_encoded_binary_matches_yaml_data() {
    // GameData freshly loaded from the yaml data files
    let loaded_game_data: GameData = GameData::load().unwrap();

    // GameData which was included binary encoded on compile-time
    let decoded_game_data: Arc<GameData> = get_game_data();

    assert_eq!(&loaded_game_data, &*decoded_game_data);
}

#[test]
fn test_monster_image_bundling() {
    let game_data = get_game_data();
    let existing_image_ids = game_data.monster_images.available_ids();
    assert!(existing_image_ids.contains(&0));
}