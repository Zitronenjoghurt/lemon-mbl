use crate::entities::stored_monster::StoredMonster;
use crate::enums::save_file_mode::SaveFileMode;
use crate::states::game_state::GameState;
use std::path::PathBuf;

#[test]
fn test_saving_loading() {
    let game_state = GameState::default();
    let test_monster = StoredMonster::create(0).unwrap();
    game_state.add_monster(test_monster);

    let test_path = PathBuf::from("./test_data");
    let bin_path = test_path.join("save.bin");
    let yaml_path = test_path.join("save.yaml");
    let json_path = test_path.join("save.json");

    game_state.save(&bin_path, SaveFileMode::Bin).unwrap();
    game_state.save(&yaml_path, SaveFileMode::Yaml).unwrap();
    game_state.save(&json_path, SaveFileMode::Json).unwrap();

    let game_state_bin = GameState::load(&bin_path).unwrap();
    let game_state_yaml = GameState::load(&yaml_path).unwrap();
    let game_state_json = GameState::load(&json_path).unwrap();

    assert_eq!(game_state, game_state_bin);
    assert_eq!(game_state, game_state_yaml);
    assert_eq!(game_state, game_state_json);
}