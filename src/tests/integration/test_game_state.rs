use crate::entities::stored_action::StoredAction;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::locale::Locale;
use crate::enums::save_file_mode::SaveFileMode;
use crate::states::game_state::GameState;
use std::path::PathBuf;

#[test]
fn test_saving_loading() {
    let game_state = GameState::default();
    let test_monster = StoredMonster::create(0).unwrap();
    let test_monster_storage_id = game_state.add_monster(test_monster);

    let test_action = StoredAction::create(0).unwrap();
    game_state.update_monster(test_monster_storage_id, |monster| monster.learn_action(test_action));

    // Very awkward access, updating the stored action data on the stored monster data, not intended to be used like this.
    // If there is ever a use-case for something like this, I will build something like update_action.
    game_state.update_monster(test_monster_storage_id, |monster| monster.get_actions_mut().get_mut(0).map(|action| action.on_use()).unwrap());

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

#[test]
fn test_i18n() {
    let mut game_state = GameState::default();

    assert_eq!(game_state.get_current_locale(), Locale::English);
    assert_eq!(game_state.get_monster_name(0).unwrap(), "Test Monster");
    assert_eq!(game_state.get_monster_description(0).unwrap(), "Its origins remain a mystery...");

    game_state.set_locale(Locale::German);
    assert_eq!(game_state.get_current_locale(), Locale::German);
    assert_eq!(game_state.get_monster_name(0).unwrap(), "Test Monster");
    assert_eq!(game_state.get_monster_description(0).unwrap(), "Der Ursprung dieses Wesens bleibt ein Rätsel...");
}