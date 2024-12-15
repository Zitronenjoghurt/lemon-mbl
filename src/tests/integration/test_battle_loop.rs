use crate::battle_logic::battle_state::BattleState;
use crate::entities::battle_monster::BattleMonster;
use crate::entities::stored_action::StoredAction;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::save_file_mode::SaveFileMode;
use crate::enums::team_side::TeamSide;
use crate::states::game_state::GameState;
use std::path::PathBuf;

#[test]
fn test_basic_damage_and_heal() {
    let mut monster_a = StoredMonster::create(0).unwrap();
    let mut monster_b = StoredMonster::create(0).unwrap();

    let test_attack = StoredAction::create(0).unwrap();
    let test_heal = StoredAction::create(1).unwrap();

    monster_a.learn_action(test_attack.clone());
    monster_b.learn_action(test_attack);

    monster_a.learn_action(test_heal.clone());
    monster_b.learn_action(test_heal);

    let battle_monster_a = BattleMonster::from(monster_a);
    let battle_monster_b = BattleMonster::from(monster_b);

    let team_a = [battle_monster_a];
    let team_b = [battle_monster_b];

    let mut battle = BattleState::new(Vec::from(team_a), Vec::from(team_b));

    battle.take_action(0, &TeamSide::TeamA, &TeamSide::TeamB, 0, 0).unwrap();
    battle.take_action(1, &TeamSide::TeamB, &TeamSide::TeamB, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    let monster_a = battle.get_monster(&TeamSide::TeamA, 0).unwrap();
    let monster_b = battle.get_monster(&TeamSide::TeamB, 0).unwrap();

    assert_eq!(monster_a.get_current_hp(), 50);
    assert_eq!(monster_a.get_action(0).unwrap().get_total_use_count(), 1);
    assert_eq!(monster_a.get_action(1).unwrap().get_total_use_count(), 0);
    assert_eq!(monster_b.get_current_hp(), 15);
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 0);
    assert_eq!(monster_b.get_action(1).unwrap().get_total_use_count(), 1);

    // Check stats
    assert_eq!(monster_a.get_damage_dealt(), 40);
    assert_eq!(monster_b.get_damage_dealt(), 0);
    assert_eq!(monster_a.get_damage_taken(), 0);
    assert_eq!(monster_b.get_damage_taken(), 40);
    assert_eq!(monster_a.get_hp_heal_given(), 0);
    assert_eq!(monster_b.get_hp_heal_given(), 5);
    assert_eq!(monster_a.get_hp_heal_received(), 0);
    assert_eq!(monster_b.get_hp_heal_received(), 5);
    assert_eq!(monster_a.get_momentum_used(), 0);
    assert_eq!(monster_b.get_momentum_used(), 0);
    assert_eq!(monster_a.get_energy_used(), 3);
    assert_eq!(monster_b.get_energy_used(), 2);
    assert_eq!(monster_a.get_hp_used(), 0);
    assert_eq!(monster_b.get_hp_used(), 0);

    assert_eq!(monster_a.get_stored_data().get_total_damage_dealt(), 40);
    assert_eq!(monster_b.get_stored_data().get_total_damage_dealt(), 0);
    assert_eq!(monster_a.get_stored_data().get_total_damage_taken(), 0);
    assert_eq!(monster_b.get_stored_data().get_total_damage_taken(), 40);
    assert_eq!(monster_a.get_stored_data().get_total_hp_heal_given(), 0);
    assert_eq!(monster_b.get_stored_data().get_total_hp_heal_given(), 5);
    assert_eq!(monster_a.get_stored_data().get_total_hp_heal_received(), 0);
    assert_eq!(monster_b.get_stored_data().get_total_hp_heal_received(), 5);
    assert_eq!(monster_a.get_stored_data().get_total_momentum_used(), 0);
    assert_eq!(monster_b.get_stored_data().get_total_momentum_used(), 0);
    assert_eq!(monster_a.get_stored_data().get_total_energy_used(), 3);
    assert_eq!(monster_b.get_stored_data().get_total_energy_used(), 2);
    assert_eq!(monster_a.get_stored_data().get_total_hp_used(), 0);
    assert_eq!(monster_b.get_stored_data().get_total_hp_used(), 0);

    // Check battle state save/load
    let mut game_state = GameState::default();
    let test_path = PathBuf::from("./test_data");
    let bin_path = test_path.join("save_with_battle.bin");
    let yaml_path = test_path.join("save_with_battle.yaml");
    let yaml_path2 = test_path.join("save_with_battle_2.yaml");
    let json_path = test_path.join("save_with_battle.json");
    game_state.set_current_battle(Some(battle));
    game_state.save(&bin_path, SaveFileMode::Bin).unwrap();
    game_state.save(&yaml_path, SaveFileMode::Yaml).unwrap();
    game_state.save(&json_path, SaveFileMode::Json).unwrap();

    let loaded_bin = GameState::load(&bin_path).unwrap();
    loaded_bin.save(&yaml_path2, SaveFileMode::Yaml).unwrap();

    assert_eq!(game_state, loaded_bin);
}