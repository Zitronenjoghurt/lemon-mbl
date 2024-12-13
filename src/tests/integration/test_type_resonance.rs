use crate::enums::damage_type::DamageType;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;

#[test]
fn test_type_resonance() {
    let game_data = get_game_data();

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![],
        &vec![]
    ), 1.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Construct],
        &vec![]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Organic],
        &vec![]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Construct, MonsterPhysicalType::Organic],
        &vec![]
    ), 4.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![],
        &vec![MonsterElementalType::Frost]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![],
        &vec![MonsterElementalType::Force]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![],
        &vec![MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 4.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Organic],
        &vec![MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 8.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Organic, MonsterPhysicalType::Construct],
        &vec![MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 16.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Ethereal],
        &vec![]
    ), 0.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Organic, MonsterPhysicalType::Construct, MonsterPhysicalType::Ethereal],
        &vec![MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 0.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Slime],
        &vec![]
    ), 0.5f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Slime],
        &vec![MonsterElementalType::Dark]
    ), 0.25f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &DamageType::Piercing,
        &vec![MonsterPhysicalType::Slime, MonsterPhysicalType::Organic],
        &vec![]
    ), 1.0f64);
}