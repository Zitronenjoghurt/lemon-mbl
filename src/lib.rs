use flate2::read::ZlibDecoder;
use once_cell::sync::Lazy;
use states::game_data::GameData;
use std::io::Read;
use std::sync::Arc;

#[cfg(feature = "dev")]
use std::sync::RwLock;

#[cfg(test)]
mod tests;
pub mod entities;
pub mod calculations;
pub mod enums;
pub mod traits;
mod utils;
mod serialization;
mod data_structures;
pub mod states;
pub mod battle_logic;

const STATIC_DATA: &[u8] = include_bytes!("../data/game_data.bin");
static GAME_DATA: Lazy<Arc<GameData>> = Lazy::new(|| {
    let mut decompressor = ZlibDecoder::new(STATIC_DATA);
    let mut decompressed_data = Vec::new();
    decompressor.read_to_end(&mut decompressed_data).unwrap();
    Arc::new(bincode::deserialize(&decompressed_data).unwrap())
});

#[cfg(feature = "dev")]
static OVERRIDE_DATA: Lazy<RwLock<Option<Arc<GameData>>>> = Lazy::new(|| RwLock::new(None));

#[cfg(not(feature = "dev"))]
pub fn get_game_data() -> Arc<GameData> {
    GAME_DATA.clone()
}

#[cfg(feature = "dev")]
pub fn get_game_data() -> Arc<GameData> {
    OVERRIDE_DATA.read().unwrap()
        .as_ref()
        .map(Arc::clone)
        .unwrap_or_else(|| GAME_DATA.clone())
}

#[cfg(feature = "dev")]
pub fn set_game_data(data: GameData) {
    *OVERRIDE_DATA.write().unwrap() = Some(Arc::new(data));
}

#[cfg(feature = "dev")]
pub fn reset_game_data() {
    *OVERRIDE_DATA.write().unwrap() = None;
}