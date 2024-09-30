use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u32,
    pub max_cpu: u32,
    pub max_ram: u32,
}
