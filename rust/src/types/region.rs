use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub enum Region {
    #[serde(rename = "de")]
    Germany,
    #[serde(rename = "us")]
    USA,
    #[serde(rename = "invalid")]
    Invalid,
}
