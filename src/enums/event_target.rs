use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum EventTarget {
    SourceMonster = 0,
    SourceTeam = 1,
    TargetMonster = 2,
    TargetTeam = 3,
}