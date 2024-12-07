use crate::get_game_data;
use lemon_mbl_game_data::game_data::GameData;
use std::sync::Arc;

#[test]
pub fn test_encoded_binary_matches_yaml_data() {
    // GameData freshly loaded from the yaml data files
    let loaded_game_data: GameData = GameData::load();

    // GameData which was included binary encoded on compile-time
    let decoded_game_data: Arc<GameData> = get_game_data();

    assert_eq!(&loaded_game_data, &*decoded_game_data);
}