use flate2::read::ZlibDecoder;
use lemon_mbl_game_data::game_data::GameData;
use once_cell::sync::Lazy;
use std::io::Read;
use std::sync::Arc;

#[cfg(test)]
mod tests;
mod entities;
mod calculations;

const STATIC_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));
static GAME_DATA: Lazy<Arc<GameData>> = Lazy::new(|| {
    let mut decompressor = ZlibDecoder::new(STATIC_DATA);
    let mut decompressed_data = Vec::new();
    decompressor.read_to_end(&mut decompressed_data).unwrap();
    Arc::new(bincode::deserialize(&*decompressed_data).unwrap())
});

pub fn get_game_data() -> Arc<GameData> {
    GAME_DATA.clone()
}