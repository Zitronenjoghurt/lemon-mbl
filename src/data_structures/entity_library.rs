use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct EntityLibrary<T>
where
    T: Clone + HasId,
{
    entities: HashMap<T::Id, T>,
}

impl<T> EntityLibrary<T>
where
    T: Clone + HasId,
{
    pub fn new() -> Self {
        EntityLibrary {
            entities: HashMap::new()
        }
    }

    pub fn add(&mut self, entity: T) {
        let id = entity.id();
        self.entities.insert(id, entity);
    }

    pub fn get(&self, id: T::Id) -> Option<&T> {
        self.entities.get(&id)
    }
}

impl<T> EntityLibrary<T>
where
    T: Clone + HasId + HasDataFileYaml + for<'de> Deserialize<'de>,
{
    pub fn from_yaml() -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_file_path = T::data_file_path();
        let contents = fs::read_to_string(yaml_file_path)?;
        let yaml_entities: HashMap<String, T> = serde_yaml::from_str(&contents)?;

        let entities = yaml_entities
            .values()
            .map(|entity| (entity.id(), entity.clone()))
            .collect();

        Ok(EntityLibrary { entities })
    }
}