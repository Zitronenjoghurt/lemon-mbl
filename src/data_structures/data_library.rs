use crate::traits::has_data_file::HasDataFileYaml;
use crate::traits::has_id::HasId;
use crate::traits::has_internal_name::HasInternalName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct DataLibrary<T>
where
    T: Clone + HasId + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    entities: HashMap<T::Id, Arc<T>>,
}

impl<T> DataLibrary<T>
where
    T: Clone + HasId + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    pub fn add(&mut self, entity: T) {
        let id = entity.id();
        self.entities.insert(id, Arc::new(entity));
    }

    pub fn get(&self, id: T::Id) -> Option<&Arc<T>> {
        self.entities.get(&id)
    }
}

impl<T> DataLibrary<T>
where
    T: Clone + HasId + HasDataFileYaml + HasInternalName + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    pub fn from_yaml() -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_file_path = T::data_file_path();
        let contents_unprocessed = fs::read_to_string(yaml_file_path)?;
        let contents = T::preprocess(contents_unprocessed);
        let yaml_entities: HashMap<String, T> = serde_yaml::from_str(&contents)?;

        let entities = yaml_entities
            .into_iter()
            .map(|(key, entity)| {
                let entity = entity.with_internal_name(key);
                (entity.id(), Arc::new(entity))
            })
            .collect();

        Ok(DataLibrary { entities })
    }
}

impl<T> Serialize for DataLibrary<T>
where
    T: Clone + HasId + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.entities.len()))?;
        for (k, v) in &self.entities {
            map.serialize_entry(k, v.as_ref())?;
        }
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for DataLibrary<T>
where
    T: Clone + HasId + Serialize + for<'d> Deserialize<'d>,
    T::Id: Serialize + for<'d> Deserialize<'d>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp_map: HashMap<T::Id, T> = HashMap::deserialize(deserializer)?;

        let entities = temp_map
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();

        Ok(DataLibrary { entities })
    }
}