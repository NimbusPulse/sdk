use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DcsSettingsPayload {
    pub initial_server_name: String,
    pub initial_server_password: String,
    pub initial_max_players: u32,
    pub enable_io: bool,
    pub enable_os: bool,
    pub enable_lfs: bool,
    pub initial_use_voice_chat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DcsSettingsUpdatePayload {
    pub enable_io: bool,
    pub enable_os: bool,
    pub enable_lfs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DcsSettings {
    pub initial_server_name: String,
    pub initial_server_password: String,
    pub initial_max_players: i32,
    pub enable_io: bool,
    pub enable_os: bool,
    pub enable_lfs: bool,
    pub initial_use_voice_chat: bool,
}
