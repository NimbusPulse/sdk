use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BillingType {
    Hourly,
    Monthly,
}

impl std::fmt::Display for BillingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BillingType::Hourly => write!(f, "hourly"),
            BillingType::Monthly => write!(f, "monthly"),
        }
    }
}

impl FromStr for BillingType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hourly" => Ok(BillingType::Hourly),
            "monthly" => Ok(BillingType::Monthly),
            _ => Err(format!("Invalid billing type: {s}")),
        }
    }
}
