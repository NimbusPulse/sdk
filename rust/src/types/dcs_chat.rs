use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct DcsChatSafe {
    pub id: u32,
    pub player_id: i32,
    pub player_name: String,
    pub message: String,
    pub unix_time: i64,
}
