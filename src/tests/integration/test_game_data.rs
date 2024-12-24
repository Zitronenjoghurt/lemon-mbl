use crate::get_game_data;
use crate::states::game_data::GameData;
use std::sync::Arc;

#[test]
fn test_encoding_decoding() {
    let loaded_game_data: GameData = GameData::load().unwrap();
    let decoded_game_data: Arc<GameData> = get_game_data();
    assert_eq!(loaded_game_data, *decoded_game_data);
}

#[test]
fn test_monster_image_bundling() {
    let game_data = get_game_data();
    let existing_image_ids = game_data.monster_images.available_ids();
    assert!(existing_image_ids.contains(&0));
}

#[cfg(feature = "dev")]
#[test]
fn test_with_cloning() {
    let data = get_game_data();
    let cloned = data.clone();
    assert_eq!(*data, *cloned);
}

#[cfg(feature = "dev")]
mod dev {
    use crate::get_game_data;
    use std::path::PathBuf;

    #[test]
    fn test_dumping() {
        let data = get_game_data();

        let dump_path = PathBuf::from("./").join("test_data").join("game_data_dump");
        data.dump(dump_path).unwrap();
    }
}