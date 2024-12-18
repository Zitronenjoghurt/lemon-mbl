use crate::get_game_data;

pub fn calculate_poison_damage(monster_vitality: u32) -> u32 {
    (monster_vitality as f64 * get_game_data().config.poison_damage_per_turn).round() as u32
}