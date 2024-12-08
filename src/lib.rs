use flate2::read::ZlibDecoder;
use once_cell::sync::Lazy;
use states::game_data::GameData;
use std::io::Read;
use std::sync::Arc;

#[cfg(test)]
mod tests;
pub mod entities;
pub mod calculations;
pub mod enums;
mod traits;
mod utils;
mod serialization;
mod data_structures;
pub mod states;

const STATIC_DATA: &[u8] = include_bytes!("../data/game_data.bin");
static GAME_DATA: Lazy<Arc<GameData>> = Lazy::new(|| {
    let mut decompressor = ZlibDecoder::new(STATIC_DATA);
    let mut decompressed_data = Vec::new();
    decompressor.read_to_end(&mut decompressed_data).unwrap();
    Arc::new(bincode::deserialize(&decompressed_data).unwrap())
});

pub fn get_game_data() -> Arc<GameData> {
    GAME_DATA.clone()
}