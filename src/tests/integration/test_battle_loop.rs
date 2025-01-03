use crate::battle_logic::battle_state::BattleState;
use crate::entities::battle_monster::BattleMonster;
use crate::entities::stored_action::StoredAction;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::save_file_mode::SaveFileMode;
use crate::enums::status_effect::StatusEffect;
use crate::enums::team_side::TeamSide;
use crate::states::game_state::GameState;
use std::path::PathBuf;

#[test]
fn test_basic_damage_and_heal() {
    let mut monster_a = StoredMonster::create(0).unwrap();
    let mut monster_b = StoredMonster::create(0).unwrap();

    let test_attack = StoredAction::create(0).unwrap();
    let test_heal = StoredAction::create(1).unwrap();
    let test_flow = StoredAction::create(2).unwrap();

    monster_a.learn_action(test_attack.clone());
    monster_b.learn_action(test_attack);

    monster_a.learn_action(test_heal.clone());
    monster_b.learn_action(test_heal);

    monster_a.learn_action(test_flow.clone());
    monster_b.learn_action(test_flow);

    let battle_monster_a = BattleMonster::from(monster_a);
    let battle_monster_b = BattleMonster::from(monster_b);

    let team_a = [battle_monster_a];
    let team_b = [battle_monster_b];

    let mut battle = BattleState::new(Vec::from(team_a), Vec::from(team_b));

    battle.take_action(2, &TeamSide::TeamA, &TeamSide::TeamA, 0, 0).unwrap();
    battle.take_action(2, &TeamSide::TeamB, &TeamSide::TeamB, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    battle.take_action(0, &TeamSide::TeamA, &TeamSide::TeamB, 0, 0).unwrap();
    battle.take_action(1, &TeamSide::TeamB, &TeamSide::TeamB, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    let monster_a = battle.get_monster(&TeamSide::TeamA, 0).unwrap();
    let monster_b = battle.get_monster(&TeamSide::TeamB, 0).unwrap();

    assert_eq!(monster_a.get_current_hp(), 50);
    assert_eq!(monster_a.get_momentum(), 5);
    assert_eq!(monster_a.get_energy(), 12);
    assert_eq!(monster_a.get_action(0).unwrap().get_total_use_count(), 1);
    assert_eq!(monster_a.get_action(1).unwrap().get_total_use_count(), 0);
    assert_eq!(monster_b.get_current_hp(), 15);
    assert_eq!(monster_b.get_momentum(), 5);
    assert_eq!(monster_b.get_energy(), 13);
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 0);
    assert_eq!(monster_b.get_action(1).unwrap().get_total_use_count(), 1);

    // Check stats
    assert_eq!(monster_a.get_stats().damage_dealt, 40);
    assert_eq!(monster_b.get_stats().damage_dealt, 0);
    assert_eq!(monster_a.get_stats().damage_taken, 0);
    assert_eq!(monster_b.get_stats().damage_taken, 40);
    assert_eq!(monster_a.get_stats().hp_heal_given, 0);
    assert_eq!(monster_b.get_stats().hp_heal_given, 5);
    assert_eq!(monster_a.get_stats().hp_heal_received, 0);
    assert_eq!(monster_b.get_stats().hp_heal_received, 5);
    assert_eq!(monster_a.get_stats().momentum_used, 0);
    assert_eq!(monster_b.get_stats().momentum_used, 0);
    assert_eq!(monster_a.get_stats().energy_used, 3);
    assert_eq!(monster_b.get_stats().energy_used, 2);
    assert_eq!(monster_a.get_stats().hp_used, 0);
    assert_eq!(monster_b.get_stats().hp_used, 0);
    assert_eq!(monster_a.get_stats().momentum_generated, 5);
    assert_eq!(monster_b.get_stats().momentum_generated, 5);
    assert_eq!(monster_a.get_stats().energy_generated, 10);
    assert_eq!(monster_b.get_stats().energy_generated, 10);
    assert_eq!(monster_a.get_stats().momentum_generated_for_others, 5);
    assert_eq!(monster_b.get_stats().momentum_generated_for_others, 5);
    assert_eq!(monster_a.get_stats().energy_generated_for_others, 0);
    assert_eq!(monster_b.get_stats().energy_generated_for_others, 0);

    // Check battle state save/load
    let mut game_state = GameState::default();
    let test_path = PathBuf::from("./test_data");
    let bin_path = test_path.join("save_with_battle.bin");
    let json_path = test_path.join("save_with_battle.json");
    let json_path2 = test_path.join("save_with_battle2.json");
    game_state.set_current_battle(Some(battle));
    game_state.save(&bin_path, SaveFileMode::Bin).unwrap();
    game_state.save(&json_path, SaveFileMode::Json).unwrap();

    let loaded_bin = GameState::load(&bin_path).unwrap();
    loaded_bin.save(&json_path2, SaveFileMode::Json).unwrap();

    assert_eq!(game_state, loaded_bin);
}

#[test]
fn test_poison() {
    let mut monster_a = StoredMonster::create(0).unwrap();
    let mut monster_b = StoredMonster::create(0).unwrap();

    let test_poison = StoredAction::create(3).unwrap();
    monster_a.learn_action(test_poison.clone());
    monster_b.learn_action(test_poison);

    let battle_monster_a = BattleMonster::from(monster_a);
    let battle_monster_b = BattleMonster::from(monster_b);

    let team_a = [battle_monster_a];
    let team_b = [battle_monster_b];

    let mut battle = BattleState::new(Vec::from(team_a), Vec::from(team_b));

    battle.take_action(0, &TeamSide::TeamA, &TeamSide::TeamB, 0, 0).unwrap();
    battle.take_action(0, &TeamSide::TeamB, &TeamSide::TeamA, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    battle.take_action(0, &TeamSide::TeamA, &TeamSide::TeamB, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    battle.process_event_queue().unwrap();

    let monster_a = battle.get_monster(&TeamSide::TeamA, 0).unwrap();
    let monster_b = battle.get_monster(&TeamSide::TeamB, 0).unwrap();
    assert_eq!(monster_a.get_current_hp(), 35);
    assert_eq!(monster_a.get_action(0).unwrap().get_total_use_count(), 2);
    assert!(!monster_a.has_status_effect(StatusEffect::Poisoned));
    assert_eq!(monster_b.get_current_hp(), 35);
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 1);
    assert!(monster_b.has_status_effect(StatusEffect::Poisoned));

    battle.process_event_queue().unwrap();
    let monster_a = battle.get_monster(&TeamSide::TeamA, 0).unwrap();
    let monster_b = battle.get_monster(&TeamSide::TeamB, 0).unwrap();
    assert_eq!(monster_a.get_current_hp(), 35);
    assert_eq!(monster_a.get_action(0).unwrap().get_total_use_count(), 2);
    assert!(!monster_a.has_status_effect(StatusEffect::Poisoned));
    assert_eq!(monster_b.get_current_hp(), 30);
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 1);
    assert!(!monster_b.has_status_effect(StatusEffect::Poisoned));

    // Check stats
    assert_eq!(monster_a.get_stats().poison_damage_taken, 15);
    assert_eq!(monster_b.get_stats().poison_damage_taken, 20);
    assert_eq!(monster_a.get_stats().times_poison_applied, 2);
    assert_eq!(monster_b.get_stats().times_poison_applied, 1);
    assert_eq!(monster_a.get_stats().times_poison_received, 1);
    assert_eq!(monster_b.get_stats().times_poison_received, 2);

    // Check battle state save/load
    let mut game_state = GameState::default();
    let test_path = PathBuf::from("./test_data");
    let bin_path = test_path.join("save_with_poison.bin");
    let json_path = test_path.join("save_with_poison.json");
    let json_path2 = test_path.join("save_with_poison2.json");
    game_state.set_current_battle(Some(battle));
    game_state.save(&bin_path, SaveFileMode::Bin).unwrap();
    game_state.save(&json_path, SaveFileMode::Json).unwrap();

    let loaded_bin = GameState::load(&bin_path).unwrap();
    loaded_bin.save(&json_path2, SaveFileMode::Json).unwrap();

    assert_eq!(game_state, loaded_bin);
}

#[test]
fn test_paralysis() {
    let mut monster_a = StoredMonster::create(0).unwrap();
    let mut monster_b = StoredMonster::create(0).unwrap();

    let test_paralysis = StoredAction::create(4).unwrap();
    monster_a.learn_action(test_paralysis.clone());
    monster_b.learn_action(test_paralysis);

    let battle_monster_a = BattleMonster::from(monster_a);
    let battle_monster_b = BattleMonster::from(monster_b);

    let team_a = [battle_monster_a];
    let team_b = [battle_monster_b];

    let mut battle = BattleState::new(Vec::from(team_a), Vec::from(team_b));
    battle.take_action(0, &TeamSide::TeamA, &TeamSide::TeamB, 0, 0).unwrap();
    battle.take_action(0, &TeamSide::TeamB, &TeamSide::TeamA, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    let monster_a = battle.get_monster(&TeamSide::TeamA, 0).unwrap();
    let monster_b = battle.get_monster(&TeamSide::TeamB, 0).unwrap();
    assert!(monster_a.has_status_effect(StatusEffect::Paralyzed));
    assert_eq!(monster_a.get_action(0).unwrap().get_total_use_count(), 0);
    assert!(!monster_b.has_status_effect(StatusEffect::Paralyzed));
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 1);

    battle.take_action(0, &TeamSide::TeamA, &TeamSide::TeamB, 0, 0).unwrap();
    battle.process_event_queue().unwrap();
    let monster_a = battle.get_monster(&TeamSide::TeamA, 0).unwrap();
    let monster_b = battle.get_monster(&TeamSide::TeamB, 0).unwrap();
    assert!(!monster_a.has_status_effect(StatusEffect::Paralyzed));
    assert_eq!(monster_a.get_action(0).unwrap().get_total_use_count(), 0);
    assert!(!monster_b.has_status_effect(StatusEffect::Paralyzed));
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 1);

    // Check stats
    assert_eq!(monster_a.get_stats().times_paralyzed_while_trying_to_act, 2);
    assert_eq!(monster_b.get_stats().times_paralyzed_while_trying_to_act, 0);
    assert_eq!(monster_a.get_stats().times_paralysis_applied, 0);
    assert_eq!(monster_b.get_stats().times_paralysis_applied, 1);
    assert_eq!(monster_a.get_stats().times_paralysis_received, 1);
    assert_eq!(monster_b.get_stats().times_paralysis_received, 0);

    // Check battle state save/load
    let mut game_state = GameState::default();
    let test_path = PathBuf::from("./test_data");
    let bin_path = test_path.join("save_with_paralysis.bin");
    let json_path = test_path.join("save_with_paralysis.json");
    let json_path2 = test_path.join("save_with_paralysis2.json");
    game_state.set_current_battle(Some(battle));
    game_state.save(&bin_path, SaveFileMode::Bin).unwrap();
    game_state.save(&json_path, SaveFileMode::Json).unwrap();

    let loaded_bin = GameState::load(&bin_path).unwrap();
    loaded_bin.save(&json_path2, SaveFileMode::Json).unwrap();

    assert_eq!(game_state, loaded_bin);
}