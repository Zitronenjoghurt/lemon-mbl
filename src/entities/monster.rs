use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    id: u16,
    hp: u16,
    attack: u16,
    defense: u16,
}

impl HasId for Monster {
    type Id = u16;

    fn get_id(&self) -> u16 {
        self.id
    }

    fn set_id(&mut self, id: u16) {
        self.id = id;
    }
}

impl Monster {
    pub fn hp(&self) -> u16 {
        self.hp
    }

    pub fn attack(&self) -> u16 {
        self.attack
    }

    pub fn defense(&self) -> u16 {
        self.defense
    }
}