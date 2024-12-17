use crate::calculations::int_bps::IntBps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PoisonEventType {
    pub chance: IntBps,
    pub damage_percent: IntBps,
    pub stackable: bool,
    #[serde(default)]
    pub successful: bool,
}