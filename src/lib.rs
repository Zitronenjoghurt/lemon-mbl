use lemon_mbl_game_data::game_data::GameData;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[cfg(test)]
mod tests;

const STATIC_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));
static GAME_DATA: Lazy<Arc<GameData>> = Lazy::new(||
    Arc::new(bincode::deserialize(STATIC_DATA).unwrap())
);

pub fn get_game_data() -> Arc<GameData> {
    GAME_DATA.clone()
}