use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DcsChat {
    pub id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub message: String,
    pub is_historical: bool,
    pub unix_time: i64,
}
