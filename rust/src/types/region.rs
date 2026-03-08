use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    #[serde(rename = "de")]
    Germany,
    #[serde(rename = "us")]
    USA,
    #[serde(rename = "invalid")]
    Invalid,
}
