use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum TypeResonance {
    SuperStrong = 0,
    Strong = 1,
    Neutral = 2,
    Weak = 3,
    SuperWeak = 4,
    Immune = 5,
}

impl TypeResonance {
    pub fn get_damage_factor(&self) -> f64 {
        match self {
            TypeResonance::SuperStrong => 4.0,
            TypeResonance::Strong => 2.0,
            TypeResonance::Neutral => 1.0,
            TypeResonance::Weak => 0.5,
            TypeResonance::SuperWeak => 0.25,
            TypeResonance::Immune => 0.0,
        }
    }
}