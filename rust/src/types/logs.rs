use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceLogRequest {
    pub last_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameServerLogRow {
    pub html: String,
    pub level: Option<String>,
    pub module: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameServerLogsResponse {
    pub html_rows: Vec<GameServerLogRow>,
    pub last_timestamp: String,
    pub error_levels: HashMap<String, i32>,
    pub modules: Vec<String>,
}
