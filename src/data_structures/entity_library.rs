use crate::traits::has_id::HasId;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct EntityLibrary<T>
where
    T: HasId + Clone,
{
    entities: HashMap<T::Id, T>,
}

impl<T> EntityLibrary<T>
where
    T: HasId + Clone,
{
    pub fn new() -> Self {
        EntityLibrary {
            entities: HashMap::new()
        }
    }
}

impl<T> EntityLibrary<T>
where
    T: HasId + Clone + for<'de> Deserialize<'de>,
{
    pub fn from_yaml(path_buf: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path_buf)?;
        let yaml_entities: HashMap<String, T> = serde_yaml::from_str(&contents)?;

        let entities = yaml_entities
            .values()
            .map(|entity| (entity.id(), entity.clone()))
            .collect();

        Ok(EntityLibrary { entities })
    }

    pub fn add(&mut self, entity: T) {
        let id = entity.id();
        self.entities.insert(id, entity);
    }

    pub fn get(&self, id: T::Id) -> Option<&T> {
        self.entities.get(&id)
    }
}