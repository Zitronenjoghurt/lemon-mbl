use flate2::write::ZlibEncoder;
use flate2::Compression;
use lemon_mbl_game_data::game_data::GameData;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let game_data = GameData::load().unwrap();
    let encoded_data: Vec<u8> = bincode::serialize(&game_data).unwrap();

    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&encoded_data).unwrap();
    let compressed_data = compressor.finish().unwrap();

    let mut data_file = File::create(dest_path).unwrap();
    data_file.write_all(&compressed_data).unwrap();
}