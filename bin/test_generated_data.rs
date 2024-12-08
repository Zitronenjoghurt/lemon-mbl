use lemon_mbl::get_game_data;
use lemon_mbl::states::game_data::GameData;
use std::sync::Arc;

fn main() {
    let loaded_game_data: GameData = GameData::load().unwrap();
    let decoded_game_data: Arc<GameData> = get_game_data();
    assert_eq!(&loaded_game_data, &*decoded_game_data);
}