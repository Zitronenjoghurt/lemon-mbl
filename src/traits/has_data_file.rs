use std::path::PathBuf;

pub trait HasDataFileJson {
    fn data_file_path() -> PathBuf;
}