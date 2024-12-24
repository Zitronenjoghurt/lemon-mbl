use crate::traits::has_data_file::HasDataFileJson;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

#[cfg_attr(feature = "dev", derive(Clone))]
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
    T: Clone + HasId + HasDataFileJson + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    pub fn from_json() -> Result<Self, Box<dyn std::error::Error>> {
        let json_file_path = T::data_file_path();
        let contents = fs::read_to_string(json_file_path)?;
        let json_entities: HashMap<T::Id, T> = serde_json::from_str(&contents)?;

        let entities = json_entities
            .into_iter()
            .map(|(id, entity)| {
                let entity = entity.with_id(id);
                (id, Arc::new(entity))
            })
            .collect();

        Ok(DataLibrary { entities })
    }
}

#[cfg(feature = "dev")]
impl<T> DataLibrary<T>
where
    T: Clone + HasId + HasDataFileJson + Serialize + for<'de> Deserialize<'de>,
    T::Id: Serialize + for<'de> Deserialize<'de>,
{
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        use std::collections::BTreeMap;

        let plain_map: BTreeMap<T::Id, T> = self.entities
            .iter()
            .map(|(k, v)| (*k, (*v).as_ref().clone()))
            .collect();

        let json = serde_json::to_string_pretty(&plain_map)?;
        Ok(json)
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