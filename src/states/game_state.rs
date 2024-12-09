use crate::data_structures::entity_collection::EntityCollection;
use crate::entities::stored_monster::StoredMonster;
use crate::enums::save_file_mode::SaveFileMode;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    stored_monsters: EntityCollection<StoredMonster>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stored_monsters: EntityCollection::new(),
        }
    }
}

impl GameState {
    pub fn add_monster(&self, monster: StoredMonster) -> u64 {
        self.stored_monsters.insert(monster)
    }

    pub fn get_monster(&self, id: u64) -> Option<StoredMonster> {
        self.stored_monsters.get(id)
    }

    pub fn update_monster<F>(&self, id: u64, update_fn: F) -> bool
    where
        F: FnOnce(&mut StoredMonster),
    {
        self.stored_monsters.update(id, update_fn)
    }

    pub fn remove_monster(&self, id: u64) -> Option<StoredMonster> {
        self.stored_monsters.remove(id)
    }

    pub fn save(&self, path: &Path, file_mode: SaveFileMode) -> Result<(), Box<dyn std::error::Error>> {
        if !Self::check_file_ending(path, file_mode) {
            return Err("File ending does not fit the given save file mode. Use either .bin, .yml, .yaml or .json.".into());
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let data_file = File::create(path)?;

        match file_mode {
            SaveFileMode::Bin => self.save_bin(data_file)?,
            SaveFileMode::Yaml => self.save_yaml(data_file)?,
            SaveFileMode::Json => self.save_json(data_file)?,
        }

        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.is_file() {
            return Err("Path does not point to a file".into());
        }

        let data_file = File::open(path)?;

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("bin") => Ok(Self::load_bin(data_file)?),
            Some("yaml" | "yml") => Ok(Self::load_yaml(data_file)?),
            Some("json") => Ok(Self::load_json(data_file)?),
            _ => Err("Unsupported file extension".into())
        }
    }

    fn save_bin(&self, mut data_file: File) -> Result<(), Box<dyn std::error::Error>> {
        let encoded_data = bincode::serialize(&self)?;
        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
        compressor.write_all(&encoded_data)?;
        let compressed_data = compressor.finish()?;
        data_file.write_all(&compressed_data)?;
        Ok(())
    }

    fn load_bin(data_file: File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut compressed_data = ZlibDecoder::new(data_file);
        let mut decompressed_data = Vec::new();
        compressed_data.read_to_end(&mut decompressed_data)?;
        let game_data = bincode::deserialize(&decompressed_data)?;
        Ok(game_data)
    }

    fn save_yaml(&self, mut data_file: File) -> Result<(), Box<dyn std::error::Error>> {
        let encoded_data = serde_yaml::to_string(&self)?;
        data_file.write_all(&encoded_data.into_bytes())?;
        Ok(())
    }

    fn load_yaml(data_file: File) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_reader(data_file)
    }

    fn save_json(&self, mut data_file: File) -> Result<(), Box<dyn std::error::Error>> {
        let encoded_data = serde_json::to_string_pretty(&self)?;
        data_file.write_all(&encoded_data.into_bytes())?;
        Ok(())
    }

    fn load_json(data_file: File) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(data_file)
    }

    fn check_file_ending(path: &Path, file_mode: SaveFileMode) -> bool {
        match file_mode {
            SaveFileMode::Bin => path.extension().is_some_and(|ext| ext == "bin"),
            SaveFileMode::Json => path.extension().is_some_and(|ext| ext == "json"),
            SaveFileMode::Yaml => path.extension().is_some_and(|ext| ext == "yaml" || ext == "yml"),
        }
    }
}