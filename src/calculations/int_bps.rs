use rand::random;
use serde::{Deserialize, Serialize};

/// Represents a decimal number in basis points (1/100th of a percent)
/// where 10000 = 100%, 5000 = 50%, etc.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntBps(pub u32);

impl IntBps {
    pub fn new(value: u32) -> Self {
        IntBps(value)
    }

    pub fn get_normalized_decimal(&self) -> f32 {
        self.0 as f32 / 10000f32
    }

    pub fn from_percent(percent: f32) -> Self {
        IntBps((percent * 100.0).round() as u32)
    }

    pub fn roll(&self) -> bool {
        let roll = random::<f32>();
        roll < (self.0 as f32 / 10000.0)
    }
}