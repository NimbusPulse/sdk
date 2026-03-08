use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrometheusSeries {
    pub values: Vec<(f64, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerResourcesResponse {
    pub cpus: Vec<PrometheusSeries>,
    pub ram: Vec<PrometheusSeries>,
    pub max_ram: usize,
    pub players: Vec<PrometheusSeries>,
    pub server_fps: Vec<PrometheusSeries>,
}
