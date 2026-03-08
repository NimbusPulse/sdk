use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemResourcesPeriod::Now => write!(f, "now"),
            SystemResourcesPeriod::Hour => write!(f, "hour"),
            SystemResourcesPeriod::Day => write!(f, "day"),
            SystemResourcesPeriod::Week => write!(f, "week"),
        }
    }
}
