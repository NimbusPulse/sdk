use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct ServerResources {
    pub cpus: Vec<f64>,
    pub memory: f64,
    pub max_memory: f64,
}
