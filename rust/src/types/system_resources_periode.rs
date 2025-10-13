use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub enum SystemResourcesPeriod {
    #[serde(rename = "now")]
    Now,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
}

impl fmt::Display for SystemResourcesPeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemResourcesPeriod::Now => write!(f, "now"),
            SystemResourcesPeriod::Hour => write!(f, "hour"),
            SystemResourcesPeriod::Day => write!(f, "day"),
            SystemResourcesPeriod::Week => write!(f, "week"),
        }
    }
}
