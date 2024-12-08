use std::path::PathBuf;

pub fn data_path() -> PathBuf {
    PathBuf::from("./data")
}

pub fn game_data_path() -> PathBuf {
    data_path().join("game_data.bin")
}

pub fn stats_path() -> PathBuf {
    data_path().join("stats")
}

pub fn monster_data_path() -> PathBuf {
    stats_path().join("monsters.yml")
}

pub fn assets_path() -> PathBuf {
    PathBuf::from("./assets")
}

pub fn monster_assets_path() -> PathBuf {
    assets_path().join("monsters")
}