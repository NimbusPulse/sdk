use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct ServerResources {
    pub cpus: Vec<CpuMetric>,
    pub max_ram: u64,
    pub ram: Vec<RamMetric>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct CpuMetric {
    pub metric: CpuMetricData,
    pub values: Vec<(f64, String)>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct CpuMetricData {
    pub core: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct RamMetric {
    pub metric: RamMetricData,
    pub values: Vec<(f64, String)>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct RamMetricData {
    pub instance: String,
    pub job: String,
    pub monitor: String,
    pub vm_uuid: String,
}
