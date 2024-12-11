use crate::battle_logic::battle_state::BattleState;
use crate::entities::battle_monster::BattleMonster;
use crate::entities::stored_action::StoredAction;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::team_side::TeamSide;

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
    assert_eq!(monster_b.get_current_hp(), 45);
    assert_eq!(monster_b.get_action(0).unwrap().get_total_use_count(), 0);
    assert_eq!(monster_b.get_action(1).unwrap().get_total_use_count(), 1);

    println!("{:?}", monster_a);
}