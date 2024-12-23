use crate::utils::directories::translations_data_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum Locale {
    English = 0,
    German = 1,
    French = 2,
    Italian = 3,
    Spanish = 4,
}

impl Locale {
    pub const ALL: [Locale; 5] = [
        Locale::English,
        Locale::German,
        Locale::French,
        Locale::Italian,
        Locale::Spanish,
    ];

    pub fn iter() -> impl Iterator<Item=Locale> {
        Self::ALL.into_iter()
    }

    /// Using the ISO 639-1 language codes
    pub fn from_language_code(language_code: &str) -> Result<Locale, ()> {
        match language_code {
            "en" => Ok(Locale::English),
            "de" => Ok(Locale::German),
            "fr" => Ok(Locale::French),
            "it" => Ok(Locale::Italian),
            "es" => Ok(Locale::Spanish),
            _ => Err(()),
        }
    }

    /// Using the ISO 639-1 language codes
    pub fn get_language_code(&self) -> &'static str {
        match self {
            Locale::English => "en",
            Locale::German => "de",
            Locale::French => "fr",
            Locale::Italian => "it",
            Locale::Spanish => "es",
        }
    }

    pub fn get_language_name(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::German => "Deutsch",
            Locale::French => "Français",
            Locale::Italian => "Italiano",
            Locale::Spanish => "Español",
        }
    }

    pub fn get_translations_path(&self) -> PathBuf {
        let file_name = format!("{}.yml", self.get_language_code());
        translations_data_path().join(file_name)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::English
    }
}