use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
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
        match s.to_lowercase().as_str() {
            "hourly" => Ok(BillingType::Hourly),
            "monthly" => Ok(BillingType::Monthly),
            _ => Err(format!("Invalid billing type: {}", s)),
        }
    }
}
