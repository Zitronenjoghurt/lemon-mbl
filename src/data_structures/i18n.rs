use crate::enums::locale::Locale;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[cfg_attr(feature = "dev", derive(Clone))]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct I18n {
    translations: HashMap<Locale, I18nEntry>,
}

impl I18n {
    pub fn get_monster_name(&self, locale: &Locale, monster_id: u16) -> Option<&str> {
        self.translations
            .get(locale)
            .and_then(|entry| entry.monsters.names.get(&monster_id))
            .map(String::as_str)
    }

    pub fn get_monster_description(&self, locale: &Locale, monster_id: u16) -> Option<&str> {
        self.translations
            .get(locale)
            .and_then(|entry| entry.monsters.descriptions.get(&monster_id))
            .map(String::as_str)
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut translations = HashMap::new();

        println!("Loading translations...");
        for locale in Locale::iter() {
            let entry = I18nEntry::load(locale)?;
            translations.insert(locale, entry);
        }
        println!("Finished loading translations.");

        Ok(Self { translations })
    }
}

#[cfg_attr(feature = "dev", derive(Clone))]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct I18nEntry {
    monsters: MonstersI18n,
}

impl I18nEntry {
    pub fn load(locale: Locale) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = locale.get_translations_path();
        if !file_path.exists() {
            println!("No translations found for locale '{}', skipping...", locale.get_language_code());
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(file_path)?;
        let entry: I18nEntry = serde_yaml::from_str(&contents)?;
        println!("Successfully loaded translations for locale '{}'.", locale.get_language_code());
        Ok(entry)
    }
}

#[cfg_attr(feature = "dev", derive(Clone))]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MonstersI18n {
    pub names: HashMap<u16, String>,
    pub descriptions: HashMap<u16, String>,
}