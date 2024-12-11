use std::path::PathBuf;

pub trait HasDataFileYaml {
    fn data_file_path() -> PathBuf;
    fn preprocess(contents: String) -> String {
        contents
    }
}