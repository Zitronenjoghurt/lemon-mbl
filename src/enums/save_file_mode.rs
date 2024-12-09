use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum SaveFileMode {
    Bin = 0,
    Yaml = 1,
    Json = 2,
}