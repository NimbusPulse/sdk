use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u32,
    pub max_cpu: u32,
    pub max_ram: u32,
}
