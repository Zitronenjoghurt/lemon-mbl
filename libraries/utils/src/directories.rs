use std::path::PathBuf;

pub fn data_path() -> PathBuf {
    PathBuf::from("./data")
}

pub fn monster_data_path() -> PathBuf {
    data_path().join("monsters.yml")
}