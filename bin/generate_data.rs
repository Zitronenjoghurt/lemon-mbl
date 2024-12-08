use flate2::write::ZlibEncoder;
use flate2::Compression;
use lemon_mbl::game_data::GameData;
use lemon_mbl::utils::directories::game_data_path;
use std::fs::File;
use std::io::Write;

fn main() {
    let game_data = GameData::load().unwrap();
    let encoded_data: Vec<u8> = bincode::serialize(&game_data).unwrap();

    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&encoded_data).unwrap();
    let compressed_data = compressor.finish().unwrap();

    let mut data_file = File::create(game_data_path()).unwrap();
    data_file.write_all(&compressed_data).unwrap();
}