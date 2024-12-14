use crate::enums::damage_type::DamageType;
use crate::enums::monster_elemental_type::MonsterElementalType;
use crate::enums::monster_physical_type::MonsterPhysicalType;
use crate::get_game_data;

#[test]
fn test_type_resonance() {
    let game_data = get_game_data();

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[],
        &[]
    ), 1.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Construct],
        &[]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Organic],
        &[]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Construct, MonsterPhysicalType::Organic],
        &[]
    ), 4.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[],
        &[MonsterElementalType::Frost]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[],
        &[MonsterElementalType::Force]
    ), 2.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[],
        &[MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 4.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Organic],
        &[MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 8.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Organic, MonsterPhysicalType::Construct],
        &[MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 16.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Ethereal],
        &[]
    ), 0.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Organic, MonsterPhysicalType::Construct, MonsterPhysicalType::Ethereal],
        &[MonsterElementalType::Force, MonsterElementalType::Frost]
    ), 0.0f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Slime],
        &[]
    ), 0.5f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Slime],
        &[MonsterElementalType::Dark]
    ), 0.25f64);

    assert_eq!(game_data.damage_types.calculate_damage_factor(
        &[DamageType::Piercing],
        &[MonsterPhysicalType::Slime, MonsterPhysicalType::Organic],
        &[]
    ), 1.0f64);
}