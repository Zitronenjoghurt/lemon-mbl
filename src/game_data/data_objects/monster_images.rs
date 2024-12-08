use crate::utils::directories::monster_assets_path;
use image::{DynamicImage, ImageError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MonsterImages(HashMap<u16, Vec<u8>>);

impl MonsterImages {
    pub fn available_ids(&self) -> Vec<u16> {
        self.0.keys().copied().collect()
    }

    pub fn get_raw(&self, id: u16) -> Option<&Vec<u8>> {
        self.0.get(&id)
    }

    pub fn get_image(&self, id: u16) -> Option<Result<DynamicImage, ImageError>> {
        self.get_raw(id).map(|bytes| {
            image::load_from_memory(bytes)
        })
    }

    pub fn save_to_file(&self, id: u16, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(bytes) = self.get_raw(id) {
            let mut file = std::fs::File::create(path)?;
            file.write_all(bytes)?;
            Ok(())
        } else {
            Err("Monster ID not found".into())
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut images = HashMap::new();

        for entry in fs::read_dir(monster_assets_path())? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            if let Some(id_str) = file_name.strip_prefix("monster_")
                .and_then(|s| s.strip_suffix(".png"))
            {
                if let Ok(id) = id_str.parse::<u16>() {
                    let contents = fs::read(entry.path())?;
                    images.insert(id, contents);
                }
            }
        }

        Ok(MonsterImages(images))
    }
}