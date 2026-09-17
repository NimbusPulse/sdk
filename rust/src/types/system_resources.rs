use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrometheusSeries {
    pub values: Vec<(f64, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUsage {
    pub id: String,
    pub name: String,
    pub kind: ResourceKind,
    pub cpu_percent: Option<PrometheusSeries>,
    pub memory_bytes: Option<PrometheusSeries>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    System,
    GameServer,
    Agent,
    Mod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerResourcesResponse {
    pub cpus: Vec<PrometheusSeries>,
    pub resources: Vec<ResourceUsage>,
    pub max_ram: usize,
    pub players: Vec<PrometheusSeries>,
    pub server_fps: Vec<PrometheusSeries>,
}
