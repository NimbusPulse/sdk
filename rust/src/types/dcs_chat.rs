use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct DcsChatSafe {
    pub id: u32,
    pub player_id: i32,
    pub player_name: String,
    pub message: String,
    pub unix_time: i64,
}
